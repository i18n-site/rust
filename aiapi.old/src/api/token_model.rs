use aok::Result;
use pos_next::PosNext;
use serde::Deserialize;

use crate::OpenAi;

// 不同账号模型名称不一样, 比如字节的豆包
#[derive(Deserialize, Debug)]
pub struct Account {
  pub token: String,
  pub model: String,
}

#[derive(Deserialize, Debug)]
pub struct Conf {
  pub api: String,
  pub temperature: String,
  pub max_tokens: usize,
  pub account: Vec<Account>,
  pub 上下文: usize,
}

#[derive(Debug)]
pub struct TokenModel {
  pub _上下文: usize,
  pub account: Vec<Account>,
  pub _url_chat_completions: String,
  pub _arg: String,
  pub _pos: PosNext,
  pub _max_tokens: usize,
}

impl TokenModel {
  pub fn loads(yml: impl AsRef<[u8]>) -> Result<Self, serde_yaml::Error> {
    let t: Conf = serde_yaml::from_slice(yml.as_ref())?;
    Ok(Self {
      _max_tokens: t.max_tokens,
      _上下文: t.上下文,
      _pos: PosNext::new(),
      account: t.account,
      _url_chat_completions: super::url_chat_completions(&t.api),
      _arg: super::temperature(&t.temperature),
    })
  }
}

impl OpenAi for TokenModel {
  fn max_tokens(&self) -> usize {
    self._max_tokens
  }

  fn url_chat_completions(&self) -> &str {
    &self._url_chat_completions
  }

  fn token_model(&self) -> (&str, &str) {
    let ac = &self.account[self._pos.next() % self.account.len()];
    (&ac.token, &ac.model)
  }

  fn arg(&self) -> &str {
    &self._arg
  }

  fn 上下文(&self) -> usize {
    self._上下文
  }
}
