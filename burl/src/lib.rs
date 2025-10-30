pub use base_x::{Alphabet, DecodeError, decode, encode};

pub const BURL: &str = "!$-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz";

pub fn e(input: impl AsRef<[u8]>) -> String {
  BURL.encode(input.as_ref())
}

pub fn d(input: &str) -> Result<Vec<u8>, DecodeError> {
  BURL.decode(input)
}
