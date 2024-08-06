use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Error(Vec<(String, String)>);

impl Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", sonic_rs::to_string(&self).unwrap())
  }
}

impl std::error::Error for Error {}

pub type Result = std::result::Result<(), Error>;

impl Error {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn ok(self) -> Result {
    if !self.0.is_empty() {
      return Err(self)?;
    }
    Ok(())
  }

  pub fn throw(key: impl Into<String>, val: impl Into<String>) -> Result {
    Err(Self(vec![(key.into(), val.into())]))?
  }

  pub fn set(&mut self, key: impl Into<String>, val: impl Into<String>) {
    self.0.push((key.into(), val.into()));
  }
}
