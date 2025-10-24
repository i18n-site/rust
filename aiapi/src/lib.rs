#![cfg_attr(docsrs, feature(doc_cfg))]

use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("json: {0}")]
  Json(#[from] sonic_rs::Error),

  #[error("{0:?}")]
  Reqwest(#[from] reqwest::Error),

  #[error("{status}\n{text}")]
  Response {
    status: reqwest::StatusCode,
    text: String,
  },

  #[cfg(feature = "from_yml")]
  #[error("yml: {0}")]
  Yml(#[from] saphyr::ScanError),

  #[cfg(feature = "from_yml")]
  #[error("{path}: {error}")]
  File { error: std::io::Error, path: String },

  #[error("API: {status}\n{text}")]
  Api {
    status: reqwest::StatusCode,
    text: String,
  },

  #[error("{0:?}")]
  Gemini(gemini::GeminiError),

  #[error("EmptyResponse: {text}")]
  EmptyResponse { text: String },

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
  fn req(
    &self,
    client: &Client,
    conf: &impl ConfTrait,
    model: &str,
    content: impl Into<String>,
  ) -> Result<RequestBuilder>;

  fn chat(
    &self,
    token: &str,
    req: &RequestBuilder,
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
