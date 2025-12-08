use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),

  #[error(transparent)]
  Ed25519(#[from] ed25519_dalek::SignatureError),
}

pub type Result<T> = std::result::Result<T, Error>;
