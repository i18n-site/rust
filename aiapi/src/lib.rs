#![feature(let_chains)]
#![feature(trait_alias)]

use std::{ops::Deref, path::Path, sync::Arc};

use aok::Result;

pub use crate::api::{Api, Baidu, EnumApi, OpenAi, Token, TokenModel};

pub mod msg;
pub use msg::Msg;

pub mod response;
pub use response::Response;
pub mod api;

pub mod role {
  pub const ASSISTANT: &str = "assistant";
  pub const USER: &str = "user";
}

#[derive(Clone, Debug)]
pub struct Ai {
  pub api: Arc<EnumApi>,
  pub name: String,
}

impl Deref for Ai {
  type Target = Arc<EnumApi>;

  fn deref(&self) -> &Self::Target {
    &self.api
  }
}

pub fn file_name<P: AsRef<Path>>(path: P) -> String {
  let path = path.as_ref();

  if let Some(file_stem) = path.file_stem() {
    if let Some(file_stem_str) = file_stem.to_str() {
      return file_stem_str.to_string();
    }
  }

  "unknown".to_string()
}

pub fn _load(
  fp: impl AsRef<std::path::Path>,
  load_api: impl Fn(&[u8]) -> Result<EnumApi>,
) -> Result<Ai> {
  let fp = fp.as_ref();
  let yml = std::fs::read(fp)?;
  Ok(Ai {
    name: file_name(fp),
    api: Arc::new(load_api(&yml)?),
  })
}

pub fn baidu(fp: impl AsRef<std::path::Path>) -> Result<Ai> {
  _load(fp, |yml| Ok(EnumApi::Baidu(Baidu::loads(yml)?)))
}

pub fn openai_token(fp: impl AsRef<std::path::Path>) -> Result<Ai> {
  _load(fp, |yml| Ok(EnumApi::Token(Token::loads(yml)?)))
}

pub fn openai_token_model(fp: impl AsRef<std::path::Path>) -> Result<Ai> {
  _load(fp, |yml| Ok(EnumApi::TokenModel(TokenModel::loads(yml)?)))
}
