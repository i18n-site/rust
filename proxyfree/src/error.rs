use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("reqwest error: {0}")]
  Reqwest(#[from] reqwest::Error),
  #[error("addr parse error: {0}")]
  AddrParse(#[from] std::net::AddrParseError),
}
