#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("JSON: {0}")]
  Json(#[from] sonic_rs::Error),

  #[error("Request: {0}")]
  Request(#[from] reqwest::Error),

  #[cfg(feature = "from_yml")]
  #[error("Yml: {0}")]
  Yml(#[from] saphyr::ScanError),

  #[cfg(feature = "from_yml")]
  #[error("{path}: {error}")]
  File { error: std::io::Error, path: String },

  #[error("API: {status}\n{text}")]
  Api {
    status: reqwest::StatusCode,
    text: String,
  },

  #[error("RateLimit: {token}\n{text}")]
  RateLimit { token: String, text: String },

  #[error("Timeout: {token}\n{text}")]
  Timeout { token: String, text: String },

  #[cfg(feature = "from_yml")]
  #[error("ConfTraitError: {0}")]
  ConfTrait(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
  pub prompt_tokens: u64,
  pub completion_tokens: u64,
  #[serde(default)]
  pub think_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
  Stop,
  Length,
  ToolCalls,
  ContentFilter,
  #[serde(other)]
  Unknown,
}

#[derive(Debug)]
pub struct ChatResult {
  pub id: String,
  pub content: String,
  pub usage: Usage,
  pub finish_reason: FinishReason,
}

pub trait AiApi {
  fn url(&self) -> &str;
  fn body(&self, conf: &impl ConfTrait, content: impl Into<String>) -> Result<String>;
  fn chat(
    &self,
    token: &str,
    body: &str,
  ) -> impl std::future::Future<Output = Result<ChatResult>> + Send;
}

pub mod conf;
pub use conf::{Conf, ConfQroq, ConfTrait, ReasoningEffort};

pub mod openai;
pub use openai::OpenAI;

pub mod token_li;
pub use token_li::TokenLi;

pub mod gemini;
pub use gemini::Gemini;

#[cfg(feature = "from_yml")]
pub mod from_yml;
