use xxhash_rust::xxh3::Xxh3;

use crate::{HASH128_LEN, hash_len_concat};

pub const MIN_HASH_SIZE: usize = 16384;

pub struct Hasher {
  hasher: Xxh3,
  buf: Vec<u8>,
  pub len: usize,
}

impl Default for Hasher {
  fn default() -> Self {
    Self::new()
  }
}

impl Hasher {
  pub fn new() -> Self {
    Self {
      hasher: crate::hasher(),
      buf: Vec::new(),
      len: 0,
    }
  }

  pub fn write(&mut self, bytes: impl AsRef<[u8]>) {
    let bytes = bytes.as_ref();

    self.len += bytes.len();

    self.buf.extend_from_slice(bytes);

    if self.buf.len() >= MIN_HASH_SIZE {
      self.hasher.update(&self.buf);
      self.buf.clear();
    }
  }

  pub fn finish(mut self) -> Vec<u8> {
    if self.len > HASH128_LEN {
      if !self.buf.is_empty() {
        self.hasher.update(&self.buf);
      }
      return hash_len_concat(self.hasher.digest128(), self.len);
    }
    self.buf
  }

  pub fn iter<Ref: AsRef<[u8]>>(iter: impl IntoIterator<Item = Ref>) -> Vec<u8> {
    let mut hasher = Self::new();
    for i in iter {
      hasher.write(i);
    }
    hasher.finish()
  }
}
