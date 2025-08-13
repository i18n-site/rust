use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{ChatResult, ConfTrait, Error, FinishReason, Result, Usage};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Msg {
  pub role: String,
  pub content: String,
}

#[derive(Serialize, Debug)]
struct ChatRequest<'a> {
  model: &'a str,
  messages: Vec<Msg>,
  temperature: f32,
}

#[derive(Deserialize, Debug)]
pub struct ChatItem {
  pub message: Msg,
  pub finish_reason: FinishReason,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
  pub choices: Vec<ChatItem>,
  pub usage: Usage,
  pub id: String,
}

#[derive(Debug)]
pub struct OpenAI {
  pub url: String,
  pub model: String,
  pub client: Client,
}

impl OpenAI {
  pub fn new(url: impl Into<String>, model: impl Into<String>) -> Self {
    Self {
      url: url.into(),
      model: model.into(),
      client: Client::new(),
    }
  }
}

impl crate::AiApi for OpenAI {
  fn body(&self, conf: &impl ConfTrait, content: impl Into<String>) -> Result<String> {
    let content = content.into();
    let mut messages = vec![];
    let system = conf.system();
    if !system.is_empty() {
      messages.push(Msg {
        role: "system".into(),
        content: system.into(),
      })
    };

    messages.push(Msg {
      role: "user".into(),
      content,
    });

    let request_body = ChatRequest {
      model: &self.model,
      messages,
      temperature: conf.temperature(),
    };

    Ok(sonic_rs::to_string(&request_body)?)
  }

  async fn chat(&self, token: &str, body: &str) -> Result<ChatResult> {
    let url = format!("{}/chat/completions", self.url);

    let req = self
      .client
      .post(&url)
      .header("Content-Type", "application/json")
      .bearer_auth(token)
      .body(body.to_owned());

    let response = req.send().await?;
    let status = response.status();
    if status.is_success() {
      let text = response.text().await?;
      // dbg!(&text);
      let chat_response: ChatResponse = sonic_rs::from_str(&text)?;
      let id = chat_response.id;
      let result = if let Some(c) = chat_response.choices.into_iter().next() {
        ChatResult {
          id,
          content: c.message.content,
          usage: chat_response.usage,
          finish_reason: c.finish_reason,
        }
      } else {
        ChatResult {
          id,
          content: "".to_string(),
          usage: chat_response.usage,
          finish_reason: FinishReason::Stop,
        }
      };
      return Ok(result);
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
    &self.url
  }
}
