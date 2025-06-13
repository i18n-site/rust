// pub mod d;
use std::borrow::Borrow;

use postcard::{from_bytes, to_allocvec, Result};
use serde::{de::DeserializeOwned, Serialize};

pub fn d<T: DeserializeOwned>(bin: impl AsRef<[u8]>) -> Result<T> {
  from_bytes(bin.as_ref())
}

pub fn e<T>(value: impl Borrow<T>) -> Result<Vec<u8>>
where
  T: Serialize + ?Sized,
{
  to_allocvec(value.borrow())
}
