use std::{cell::UnsafeCell, ops::Deref};

use rand::{Rng, SeedableRng, rngs::StdRng};

pub fn random() -> usize {
  let mut rng = StdRng::from_rng(&mut rand::rng());
  rng.random::<u64>() as usize
}

#[derive(Debug)]
pub struct PosNext {
  pub pos: UnsafeCell<usize>,
}

unsafe impl Sync for PosNext {}
unsafe impl Send for PosNext {}

impl Deref for PosNext {
  type Target = usize;
  fn deref(&self) -> &Self::Target {
    unsafe { &*self.pos.get() }
  }
}

impl Default for PosNext {
  fn default() -> Self {
    Self::new()
  }
}

impl PosNext {
  pub fn new() -> Self {
    Self {
      pos: UnsafeCell::new(random()),
    }
  }

  pub fn get<'a, T>(&self, li: &'a [T]) -> &'a T {
    let pos = self.next() % li.len();
    &li[pos]
  }

  pub fn next(&self) -> usize {
    let posptr = self.pos.get();
    let pos = unsafe { *posptr }.wrapping_add(1);
    unsafe {
      *posptr = pos;
    }
    pos
  }
}
