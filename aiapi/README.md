# aiapi

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]
#![feature(doc_cfg)]

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
```

## About

This project is an open-source component of [i18n.site ⋅ Internationalization Solution](https://i18n.site).

* [i18 : MarkDown Command Line Translation Tool](https://i18n.site/i18)

  The translation perfectly maintains the Markdown format.

  It recognizes file changes and only translates the modified files.

  The translated Markdown content is editable; if you modify the original text and translate it again, manually edited translations will not be overwritten (as long as the original text has not been changed).

* [i18n.site : MarkDown Multi-language Static Site Generator](https://i18n.site/i18n.site)

  Optimized for a better reading experience

## 关于

本项目为 [i18n.site ⋅ 国际化解决方案](https://i18n.site) 的开源组件。

* [i18 : MarkDown 命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖 （ 如果这段原文没有被修改 ）。

* [i18n.site : MarkDown 多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。
