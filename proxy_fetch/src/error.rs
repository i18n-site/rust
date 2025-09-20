pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  UrlParse(#[from] url::ParseError),
  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),
  #[error(transparent)]
  Base64Decode(#[from] base64::DecodeError),
  #[error(transparent)]
  SsConnector(#[from] reqwest_ss_proxy::error::SsConnectorError),
}
