use aok::Result;
use pos_next::PosNext;
use serde::Deserialize;

use crate::OpenAi;

// 模型名字不变
#[derive(Deserialize, Debug)]
pub struct Conf {
  pub api: String,
  pub temperature: String,
  pub model: String,
  pub max_tokens: usize,
  pub 上下文: usize,
  pub token: Vec<String>,
}

#[derive(Debug)]
pub struct Token {
  pub _pos: PosNext,
  pub _token_li: Vec<String>,
  pub _url_chat_completions: String,
  pub _arg: String,
  pub model: String,
  pub _max_tokens: usize,
  pub _上下文: usize,
}

impl Token {
  pub fn loads(yml: impl AsRef<[u8]>) -> Result<Self, serde_yaml::Error> {
    let conf: Conf = serde_yaml::from_slice(yml.as_ref())?;

    Ok(Self {
      _pos: PosNext::new(),
      _token_li: conf.token,
      _url_chat_completions: super::url_chat_completions(&conf.api),
      _arg: format!(r#""temperature":{}"#, conf.temperature),
      model: conf.model,
      _上下文: conf.上下文,
      _max_tokens: conf.max_tokens,
    })
  }
}

impl OpenAi for Token {
  fn url_chat_completions(&self) -> &str {
    &self._url_chat_completions
  }

  fn 上下文(&self) -> usize {
    self._上下文
  }

  fn token_model(&self) -> (&str, &str) {
    (
      &self._token_li[self._pos.next() % self._token_li.len()],
      &self.model,
    )
  }

  fn max_tokens(&self) -> usize {
    self._max_tokens
  }

  fn arg(&self) -> &str {
    &self._arg
  }
}
