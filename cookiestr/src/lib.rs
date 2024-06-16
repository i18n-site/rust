use base_x::DecodeError;

pub const COOKIE_SAFE_CHAR: &str =
  "!#$%&'()*+-./0123456789:<>?@ABDEFGHIJKLMNQRSTUVXYZ[]^_`abdefghijklmnqrstuvxyz{|}~";

pub fn d(s: &str) -> Result<Box<[u8]>, DecodeError> {
  Ok(base_x::decode(COOKIE_SAFE_CHAR, s)?.into())
}

pub fn e(li: impl AsRef<[u8]>) -> String {
  base_x::encode(COOKIE_SAFE_CHAR, li.as_ref())
}
