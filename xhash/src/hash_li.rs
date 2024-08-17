use std::ops::{Deref, DerefMut};

use crate::xhash;

#[derive(Debug, Clone)]
pub struct HashLi(pub Vec<Vec<u8>>);

impl Deref for HashLi {
  type Target = Vec<Vec<u8>>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for HashLi {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl HashLi {
  pub fn new<B: AsRef<[u8]>>(li: impl IntoIterator<Item = B>) -> Self {
    let li = li.into_iter().map(|b| xhash(b.as_ref())).collect();
    Self(li)
  }
}
