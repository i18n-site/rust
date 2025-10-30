use reqwest::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("{msg}\n{err}")]
  DecodeError { msg: String, err: sonic_rs::Error },
  #[error("{status}: {url}: {msg}")]
  RequestError {
    status: StatusCode,
    url: String,
    msg: String,
  },
}
