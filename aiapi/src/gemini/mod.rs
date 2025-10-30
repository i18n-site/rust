use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::{ChatResult, Error, FinishReason, ReasoningEffort, Result, Usage, conf::ConfTrait};

mod error;
pub use error::GeminiError;

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
struct ThinkingConfig {
  thinking_budget: i64,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct GenerationConfig {
  temperature: f32,
  #[serde(skip_serializing_if = "Option::is_none")]
  thinking_config: Option<ThinkingConfig>,
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
pub struct Gemini;

pub const URL: &str = "https://generativelanguage.googleapis.com/v1beta";

impl crate::AiApi for Gemini {
  fn req(
    &self,
    client: &Client,
    conf: &impl ConfTrait,
    model: &str,
    content: impl Into<String>,
  ) -> Result<RequestBuilder> {
    let system = conf.system();

    let request_body = GeminiRequest {
      contents: vec![Content {
        parts: vec![Part {
          text: content.into(),
        }],
      }],
      generation_config: GenerationConfig {
        temperature: conf.temperature(),
        thinking_config: if model.starts_with("gemma-3") {
          None
        } else {
          Some(ThinkingConfig {
            thinking_budget: match conf.reasoning_effort() {
              ReasoningEffort::None => {
                if model == "gemini-2.5-pro" {
                  128
                } else {
                  0
                }
              }
              ReasoningEffort::Default => -1,
              ReasoningEffort::Minimal => 128,
              ReasoningEffort::Low => 4096,
              ReasoningEffort::Medium => 16000,
              ReasoningEffort::High => 32768,
            },
          })
        },
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
    let url = format!("{URL}/models/{}:generateContent", model);
    let body = sonic_rs::to_string(&request_body)?;
    let req = client
      .post(&url)
      .header("Content-Type", "application/json")
      .body(body.to_owned());
    Ok(req)
  }

  async fn chat(&self, token: &str, req: &RequestBuilder) -> Result<ChatResult> {
    let mut response;
    let mut status;
    let mut text;

    let mut retry = 6;
    loop {
      let req = req.try_clone().unwrap().header("X-goog-api-key", token);

      response = req.send().await?;
      status = response.status();
      text = response.text().await?;

      if status.is_success() {
        let chat_response: GeminiResponse = sonic_rs::from_str(&text)?;
        let usage_metadata = &chat_response.usage_metadata;
        for c in chat_response.candidates {
          if !c.content.parts.is_empty() {
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
          }
        }
        if retry > 0 {
          retry -= 1;
        } else {
          return Err(Error::EmptyResponse { text });
        }
      } else {
        break;
      }
    }

    match sonic_rs::from_str(&text) {
      Ok::<GeminiError, _>(err) => {
        return Err(Error::Gemini(err));
      }
      Err(err) => {
        log::error!("{err}")
      }
    };
    Err(Error::Response { status, text })
  }

  fn url(&self) -> &str {
    URL
  }
}
