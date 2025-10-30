// 错误都用 thiserror 在 src/error.rs 中定义
// Errors are defined in src/error.rs using thiserror
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Invalid URI: {0}")]
  InvalidUri(String),

  #[error("URI is missing a host: {0}")]
  MissingHost(String),
}
