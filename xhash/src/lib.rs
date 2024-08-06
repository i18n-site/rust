#![feature(doc_cfg)]

use xxhash_rust::xxh3::{Xxh3, Xxh3Builder};

// pub use gxhash::{self, HashMap, HashSet};

pub const SEED: u64 = 100000020240803;
pub const SECRET: [u8; 192] = xxhash_rust::const_xxh3::const_custom_default_secret(SEED);
pub const HASH128_LEN: usize = 16;

pub fn hasher() -> Xxh3 {
  Xxh3Builder::new().with_secret(SECRET).build()
}

// pub fn hash32(bin: &[u8]) -> u32 {
//   gxhash::gxhash32(bin, SEED)
// }
//
// pub fn hash64(bin: &[u8]) -> u64 {
//   gxhash::gxhash64(bin, SEED)
// }
//

pub fn hash64(bin: impl AsRef<[u8]>) -> u64 {
  xxhash_rust::xxh3::xxh3_64_with_secret(bin.as_ref(), &SECRET)
}

pub fn hash128(bin: impl AsRef<[u8]>) -> u128 {
  xxhash_rust::xxh3::xxh3_128_with_secret(bin.as_ref(), &SECRET)
}

pub fn hash_len_concat(hash: u128, len: usize) -> Vec<u8> {
  [
    &hash.to_le_bytes()[..],
    &intbin::u64_bin((len - HASH128_LEN) as _)[..],
  ]
  .concat()
}

#[cfg(feature = "xhash")]
pub fn xhash(bin: impl AsRef<[u8]>) -> Vec<u8> {
  let bin = bin.as_ref();
  let len = bin.len();
  if len > HASH128_LEN {
    return hash_len_concat(hash128(bin), len);
  }
  bin.into()
}

#[cfg(feature = "hasher")]
mod hasher;
#[cfg(feature = "hasher")]
pub use hasher::Hasher;

#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "hash_li")]
mod hash_li;

#[cfg(feature = "hash_li")]
pub use hash_li::HashLi;

// #[doc(cfg(feature = "hash_li"))]
// #[cfg(feature = "speedy")]
// #[doc(cfg(feature = "speedy"))]
// #[derive(speedy::Readable, speedy::Writable)]
// #[cfg(feature = "bin_li")]
// #[doc(cfg(feature = "bin_li"))]
// pub struct BinLi(pub Vec<Box<[u8]>>);
// #[cfg(feature = "hasher")]
// mod hasher;
//
// #[cfg(feature = "hasher")]
// #[doc(cfg(feature = "hasher"))]
// pub use hasher::Hasher;
