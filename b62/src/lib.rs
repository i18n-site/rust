pub use base_x::{decode, encode, Alphabet, DecodeError};

pub const B62: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn e(input: impl AsRef<[u8]>) -> String {
  B62.encode(input.as_ref())
}

pub fn d(input: &str) -> Result<Vec<u8>, DecodeError> {
  B62.decode(input)
}
