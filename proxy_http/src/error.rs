use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Hyper: {0}")]
  Hyper(#[from] hyper::Error),
  #[error("Io: {0}")]
  Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
