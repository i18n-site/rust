pub use gxhash::{self, GxHashMap as HashMap, GxHashSet as HashSet};

pub const SEED: i64 = 212370440130137963;

pub fn hash32(input: &[u8]) -> u32 {
  gxhash::gxhash32(input, SEED)
}

pub fn hash64(input: &[u8]) -> u64 {
  gxhash::gxhash64(input, SEED)
}

pub fn hash128(input: &[u8]) -> u128 {
  gxhash::gxhash128(input, SEED)
}
