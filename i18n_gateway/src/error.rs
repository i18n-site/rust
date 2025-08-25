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

  #[error("Hyper: {0}")]
  Hyper(#[from] hyper::Error),

  #[error("HTTP: {0}")]
  Http(#[from] http::Error),

  #[error("HyperUtil: {0}")]
  HyperUtil(String),

  #[error("Rustls: {0}")]
  Rustls(#[from] rustls::Error),

  #[error("No Host")]
  NoHost,

  #[error("s2n_quic: {0}")]
  S2nQuic(#[from] s2n_quic::provider::tls::s2n_tls::error::Error),

  #[error("s2n_quic provider: {0}")]
  S2nQuicProvider(#[from] s2n_quic::provider::Error),

  #[error("Invalid Host: {0}")]
  InvalidHost(#[from] http::uri::InvalidUri),

  #[error("Upstream Not Found")]
  UpstreamNotFound,

  #[error("TokioJoin: {0}")]
  TokioJoin(#[from] tokio::task::JoinError),

  #[error("H3 Connection: {0}")]
  H3Connection(#[from] h3::error::ConnectionError),

  #[error("H3 Stream: {0}")]
  H3Stream(#[from] h3::error::StreamError),

  #[error("BodyCollect: {0}")]
  BodyCollect(String),

  #[error("Reqwest: {0}")]
  Reqwest(#[from] reqwest::Error),
}

impl From<std::convert::Infallible> for Error {
  fn from(_: std::convert::Infallible) -> Self {
    unreachable!()
  }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
