use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{ChatResult, Error, FinishReason, Result, Usage, conf::ConfTrait};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Part {
  text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Content {
  #[serde(default)]
  parts: Vec<Part>,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct GenerationConfig {
  temperature: f32,
}

#[derive(Serialize, Debug, Clone)]
struct SystemInstruction {
  parts: Vec<Part>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GeminiRequest {
  contents: Vec<Content>,
  generation_config: GenerationConfig,
  #[serde(skip_serializing_if = "Option::is_none")]
  system_instruction: Option<SystemInstruction>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Candidate {
  content: Content,
  finish_reason: String,
  // safety_ratings, token_count, etc. are ignored for now
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct UsageMetadata {
  #[serde(default)]
  prompt_token_count: u64,
  #[serde(default)]
  candidates_token_count: u64,
  #[serde(default)]
  thoughts_token_count: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GeminiResponse {
  #[serde(default)]
  candidates: Vec<Candidate>,
  #[serde(default)]
  usage_metadata: UsageMetadata,
  response_id: String,
}

impl From<String> for FinishReason {
  fn from(s: String) -> Self {
    match s.as_str() {
      "STOP" => FinishReason::Stop,
      "MAX_TOKENS" => FinishReason::Length,
      "SAFETY" => FinishReason::ContentFilter,
      _ => FinishReason::Unknown,
    }
  }
}

impl From<Content> for String {
  fn from(content: Content) -> Self {
    content
      .parts
      .into_iter()
      .map(|p| p.text)
      .collect::<Vec<_>>()
      .join("")
  }
}

#[derive(Debug)]
pub struct Gemini {
  pub model: String,
  pub client: Client,
}

impl Gemini {
  pub fn new(model: impl Into<String>) -> Self {
    Self {
      model: model.into(),
      client: Client::new(),
    }
  }
}

pub const URL: &str = "https://generativelanguage.googleapis.com/v1beta";

impl crate::AiApi for Gemini {
  fn body(&self, conf: &impl ConfTrait, content: impl Into<String>) -> Result<String> {
    let system = conf.system();

    let request_body = GeminiRequest {
      contents: vec![Content {
        parts: vec![Part {
          text: content.into(),
        }],
      }],
      generation_config: GenerationConfig {
        temperature: conf.temperature(),
      },
      system_instruction: if !system.is_empty() {
        Some(SystemInstruction {
          parts: vec![Part {
            text: system.to_string(),
          }],
        })
      } else {
        None
      },
    };
    Ok(sonic_rs::to_string(&request_body)?)
  }

  async fn chat(&self, token: &str, body: &str) -> Result<ChatResult> {
    let url = format!("{URL}/models/{}:generateContent", self.model);

    let mut response;
    let mut status;
    let mut text;

    loop {
      let req = self
        .client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("X-goog-api-key", token)
        .body(body.to_owned());

      response = req.send().await?;
      status = response.status();

      if status.is_success() {
        text = response.text().await?;
        dbg!(&text);
        let chat_response: GeminiResponse = sonic_rs::from_str(&text)?;
        let usage_metadata = &chat_response.usage_metadata;
        if let Some(c) = chat_response.candidates.into_iter().next() {
          return Ok(ChatResult {
            id: chat_response.response_id,
            content: c.content.into(),
            usage: Usage {
              prompt_tokens: usage_metadata.prompt_token_count,
              completion_tokens: usage_metadata.candidates_token_count,
              think_tokens: usage_metadata.thoughts_token_count,
            },
            finish_reason: c.finish_reason.into(),
          });
        };
        tracing::warn!("gemini {token} {text}")
      } else {
        break;
      }
    }

    let text = response.text().await?;
    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
      return Err(Error::RateLimit {
        token: token.into(),
        text,
      });
    }
    if status == reqwest::StatusCode::GATEWAY_TIMEOUT {
      return Err(Error::Timeout {
        token: token.into(),
        text,
      });
    }

    let current_error = Error::Api { status, text };
    Err(current_error)
  }

  fn url(&self) -> &str {
    URL
  }
}
