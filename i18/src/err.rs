use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Err {
  #[error("{0} : {1}")]
  Conf(PathBuf, serde_yaml::Error),

  // #[error("tran error {0} : {1}")]
  // Tran(u16, String),

  // 后台返回的错误, 比如: 欠费
  #[error("api error {code} : {msg}")]
  Api { code: i32, msg: String },

  #[error("TOKEN ERROR :\nGET IT FROM https://i18n.site/token\nwrite ~/.config/i18n.site.yml :\n  token: YOUR_TOKEN\nor set env I18N_SITE_TOKEN\n")]
  Token,
}

impl Err {
  pub fn is_exit(&self) -> bool {
    matches!(self, Self::Token)
    // matches!(self, Self::Token | Self::Api(_))
  }
}
