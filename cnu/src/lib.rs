#![feature(doc_cfg)]

#[cfg(feature = "lang")]
mod conv;

#[cfg(feature = "lang")]
#[doc(cfg(feature = "lang"))]
pub use conv::{conv, Conv, ConvEnum};

#[cfg(feature = "f2j")]
pub mod f2j;

#[cfg(feature = "j2f")]
pub mod j2f;

#[cfg(feature = "f2j")]
pub fn f2j(s: impl AsRef<str>) -> String {
  let s = s.as_ref();
  s.chars()
    .map(|c| f2j::F2J.get(&c).copied().unwrap_or(c))
    .collect()
}

#[cfg(feature = "j2f")]
pub fn j2f(s: impl AsRef<str>) -> String {
  let s = s.as_ref();
  s.chars()
    .map(|c| j2f::J2F.get(&c).copied().unwrap_or(c))
    .collect()
}

pub fn is_cn_char(i: char) -> bool {
  let i = i as u32;
  for [b, e] in [
    [0x4E00, 0x9FA5],
    [0x9FA6, 0x9FCB],
    [0x3400, 0x4DB5],
    [0x20000, 0x2A6D6],
    [0x2A700, 0x2B734],
    [0x2B740, 0x2B81D],
    [0x2F00, 0x2FD5],
    [0x2E80, 0x2EF3],
    [0xF900, 0xFAD9],
    [0x2F800, 0x2FA1D],
    [0xE815, 0xE86F],
    [0xE400, 0xE5E8],
    [0xE600, 0xE6CF],
    [0x31C0, 0x31E3],
    [0x2FF0, 0x2FFB],
    [0x3105, 0x3120],
    [0x31A0, 0x31BA],
  ] {
    if i >= b && i <= e {
      return true;
    }
  }
  if i == 0x3007 {
    return true;
  }
  false
}
