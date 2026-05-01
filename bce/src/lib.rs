use bitcode::{Decode, Encode};

pub fn e<T: Encode>(t: &T) -> aok::Result<Vec<u8>> {
  Ok(bitcode::encode(t))
}

pub fn d<T: for<'a> Decode<'a>>(t: impl AsRef<[u8]>) -> aok::Result<T> {
  Ok(bitcode::decode(t.as_ref())?)
}
