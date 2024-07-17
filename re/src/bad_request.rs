use std::{fmt, fmt::Display};

#[derive(Debug, Default)]
pub struct Error(String);

impl Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", self.0)
  }
}

impl std::error::Error for Error {}

pub type Result = std::result::Result<(), Error>;

pub fn new(err: impl std::error::Error) -> Error {
  Error(err.to_string())
}
