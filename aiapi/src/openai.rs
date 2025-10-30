use std::collections::HashMap;

use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use sonic_rs::to_value;

use crate::{ChatResult, ConfTrait, Error, FinishReason, Result, Usage, conf::ReasoningEffort};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Msg {
  pub role: String,
  pub content: String,
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
}

impl OpenAI {
  pub fn new(url: impl Into<String>) -> Self {
    Self { url: url.into() }
  }
}

impl crate::AiApi for OpenAI {
  fn req(
    &self,
    client: &Client,
    conf: &impl ConfTrait,
    model: &str,
    content: impl Into<String>,
  ) -> Result<RequestBuilder> {
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

    let mut map = HashMap::new();

    map.insert("model", to_value(model)?);
    map.insert("messages", to_value(&messages)?);
    map.insert("temperature", to_value(&conf.temperature())?);

    let reasoning_effort = conf.reasoning_effort();
    if reasoning_effort != ReasoningEffort::Default {
      map.insert("reasoning_effort", to_value(&reasoning_effort)?);
    }
    let url = format!("{}/chat/completions", self.url);

    let req = client
      .post(&url)
      .header("Content-Type", "application/json")
      .body(sonic_rs::to_string(&map)?);

    Ok(req)
  }

  async fn chat(&self, token: &str, req: &RequestBuilder) -> Result<ChatResult> {
    let req = req.try_clone().unwrap().bearer_auth(token);

    let response = req.send().await?;
    let status = response.status();
    let text = response.text().await?;
    if status.is_success() {
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
        return Err(Error::EmptyResponse { text });
      };
      return Ok(result);
    }

    Err(Error::Response { status, text })
  }

  fn url(&self) -> &str {
    &self.url
  }
}
