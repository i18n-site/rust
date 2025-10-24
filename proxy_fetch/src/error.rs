pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  UrlParse(#[from] url::ParseError),
  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),
  #[error(transparent)]
  ReqwestMiddleware(#[from] reqwest_middleware::Error),
  #[error(transparent)]
  Base64Decode(#[from] base64::DecodeError),
  #[error(transparent)]
  Proxy(#[from] reqwest_proxy::Error),
  #[error(transparent)]
  Io(#[from] std::io::Error),
}
