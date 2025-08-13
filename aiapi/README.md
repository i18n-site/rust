# aiapi

```rust
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

  #[error("API: {status}\n{text}")]
  Api {
    status: reqwest::StatusCode,
    text: String,
  },

  #[error("RateLimit: {token}\n{text}")]
  RateLimit { token: String, text: String },

  #[error("Timeout: {token}\n{text}")]
  Timeout { token: String, text: String },

  #[cfg(feature = "from_env")]
  #[error("MissEnv : {0}")]
  MissEnv(String),

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
pub use conf::{Conf, ConfNoThink, ConfTrait};

pub mod openai;
pub use openai::OpenAI;

pub mod token_li;
pub use token_li::TokenLi;

pub mod gemini;
pub use gemini::Gemini;

#[cfg(feature = "from_yml")]
pub mod openai_from_yml;
#[cfg(feature = "from_yml")]
pub use openai_from_yml::openai_from_yml;

#[cfg(feature = "from_yml")]
pub mod gemini_from_yml;
#[cfg(feature = "from_yml")]
pub use gemini_from_yml::gemini_from_yml;
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

* [i18 :  MarkDown命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖（如果这段原文没有被修改）。

* [i18n.site : MarkDown多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。
