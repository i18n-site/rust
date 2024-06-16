use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Err {
  #[error("{0} : {1}")]
  Conf(PathBuf, serde_yaml::Error),

  #[error("tran error {0} : {1}")]
  Tran(u16, String),

  #[error("api error : {0}")]
  Api(i32),

  #[error("TOKEN ERROR :\nGET IT FROM https://i18n.site/token\nwrite ~/.config/i18n.site.yml `token: YOUR_TOKEN`\n or\nset env I18N_SITE_TOKEN\n")]
  Token,
}

impl Err {
  pub fn is_exit(&self) -> bool {
    matches!(self, Self::Token | Self::Api(_))
  }
}
