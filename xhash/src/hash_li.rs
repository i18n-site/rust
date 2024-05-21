use std::ops::Deref;

use crate::{xhash, BinLi, Hasher};

impl Deref for BinLi {
  type Target = Vec<Box<[u8]>>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub struct HashLi {
  pub li: BinLi,
  pub hash: Vec<u8>,
}

impl HashLi {
  pub fn new<T: IntoIterator<Item = B>, B: AsRef<[u8]>>(iter: T) -> Self {
    let mut li = Vec::new();
    let mut hasher = Hasher::new();
    for i in iter {
      let i = i.as_ref();
      let hash = xhash(i);
      hasher.write(&hash);
      li.push(hash);
    }
    Self {
      li: BinLi(li),
      hash: hasher.finish(),
    }
  }
}
