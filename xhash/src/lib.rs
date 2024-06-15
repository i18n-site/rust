#![feature(doc_cfg)]

pub use gxhash::{self, HashMap, HashSet};

pub const SEED: i64 = 0;

pub fn hash32(input: &[u8]) -> u32 {
  gxhash::gxhash32(input, SEED)
}

pub fn hash64(input: &[u8]) -> u64 {
  gxhash::gxhash64(input, SEED)
}

pub fn hash128(input: &[u8]) -> u128 {
  gxhash::gxhash128(input, SEED)
}

pub const HASH128_LEN: usize = 16;

macro_rules! xhash {
  ($hash:expr,$len:expr) => {
    [
      &$hash.to_le_bytes()[..],
      &intbin::u64_bin(($len - HASH128_LEN) as _)[..],
    ]
    .concat()
    .into()
  };
}

#[cfg(feature = "xhash")]
#[doc(cfg(feature = "xhash"))]
pub fn xhash(input: impl AsRef<[u8]>) -> Box<[u8]> {
  let input = input.as_ref();

  let len = input.len();

  if len > HASH128_LEN {
    return xhash!(hash128(input), len);
  }

  input.into()
}

#[cfg(feature = "hasher")]
mod hasher;

#[cfg(feature = "hasher")]
#[doc(cfg(feature = "hasher"))]
pub use hasher::Hasher;

#[cfg(feature = "hash_li")]
mod hash_li;

#[cfg(feature = "hash_li")]
#[doc(cfg(feature = "hash_li"))]
pub use hash_li::HashLi;

#[cfg(feature = "speedy")]
#[doc(cfg(feature = "speedy"))]
#[derive(speedy::Readable, speedy::Writable)]
#[cfg(feature = "bin_li")]
#[doc(cfg(feature = "bin_li"))]
pub struct BinLi(pub Vec<Box<[u8]>>);
