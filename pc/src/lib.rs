use std::borrow::Borrow;

pub use postcard::Result;
use postcard::{from_bytes, to_allocvec};
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
