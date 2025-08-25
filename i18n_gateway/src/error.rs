use hyper::http;
use rustls;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("CertParse: {0}")]
  CertParse(String),

  #[error("CertNotFound")]
  CertNotFound(String),

  #[error("PrivateKeyNotFound")]
  PrivateKeyNotFound,

  #[error("IO: {0}")]
  Io(#[from] std::io::Error),

  #[error("HTTP: {0}")]
  Http(#[from] http::Error),

  #[error("Rustls: {0}")]
  Rustls(#[from] rustls::Error),

  #[error("No Host")]
  NoHost,

  #[error("Invalid Host: {0}")]
  InvalidHost(#[from] http::uri::InvalidUri),

  #[error("Upstream Not Found")]
  UpstreamNotFound,

  #[error("TokioJoin: {0}")]
  TokioJoin(#[from] tokio::task::JoinError),

  #[error("BodyCollect: {0}")]
  BodyCollect(String),
}

impl From<std::convert::Infallible> for Error {
  fn from(_: std::convert::Infallible) -> Self {
    unreachable!()
  }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
