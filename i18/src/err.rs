use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Err {
  #[error("{0} : {1}")]
  Conf(PathBuf, serde_yaml::Error),

  #[error("api error {0} : {1}")]
  Api(u16, String),

  #[error("token error")]
  Token,
}
