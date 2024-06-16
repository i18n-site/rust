use crate::{HASH128_LEN, SEED};

pub const MIN_HASH_SIZE: usize = 16384;

pub struct Hasher {
  hasher: gxhash::GxHasher,
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
      hasher: gxhash::GxHasher::with_seed(SEED),
      buf: Vec::new(),
      len: 0,
    }
  }

  pub fn write(&mut self, bytes: impl AsRef<[u8]>) {
    let bytes = bytes.as_ref();

    self.len += bytes.len();

    self.buf.extend_from_slice(bytes);

    if self.buf.len() >= MIN_HASH_SIZE {
      use std::hash::Hasher;
      self.hasher.write(&self.buf);
      self.buf.clear();
    }
  }

  pub fn finish(mut self) -> Vec<u8> {
    if self.len > HASH128_LEN {
      if !self.buf.is_empty() {
        use std::hash::Hasher;
        self.hasher.write(&self.buf);
      }
      return xhash!(self.hasher.finish_u128(), self.len);
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
