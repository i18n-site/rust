use std::fmt;

use num::cast::AsPrimitive;

pub const R62: [u8; 62] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const R36: [u8; 36] = *b"0123456789abcdefghijklmnopqrstuvwxyz";

pub fn _e<const N: usize>(n: impl AsPrimitive<usize>, base: [u8; N]) -> String {
  let mut result = Vec::new();
  let mut n: usize = n.as_();
  loop {
    let rem = n % N;
    result.push(base[rem] as char);
    n /= N;

    if n == 0 {
      break;
    }
  }

  result.iter().rev().collect()
}

pub fn e36(n: impl AsPrimitive<usize>) -> String {
  _e(n, R36)
}

pub fn e(n: impl AsPrimitive<usize>) -> String {
  _e(n, R62)
}

#[derive(Debug)]
pub enum Error {
  InvalidChar(char),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::InvalidChar(c) => write!(f, "InvalidChar: {c}"),
    }
  }
}

impl std::error::Error for Error {}

pub fn d(s: impl AsRef<str>) -> Result<usize, Error> {
  let s = s.as_ref();
  let base: usize = 62;
  let mut result = 0;

  for (i, c) in s.chars().rev().enumerate() {
    let value = if c.is_ascii_digit() {
      c as usize - '0' as usize
    } else if c.is_ascii_uppercase() {
      c as usize - 'A' as usize + 10
    } else if c.is_ascii_lowercase() {
      c as usize - 'a' as usize + 36
    } else {
      return Err(Error::InvalidChar(c));
    };
    result += value * base.pow(i as u32);
  }

  Ok(result)
}
