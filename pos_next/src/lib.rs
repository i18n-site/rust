use std::{cell::UnsafeCell, ops::Deref};

#[cfg(feature = "rand")]
pub fn random() -> usize {
  use rand::{Rng, SeedableRng, rngs::StdRng};
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

impl PosNext {
  pub fn new(n: usize) -> Self {
    Self {
      pos: UnsafeCell::new(n),
    }
  }

  #[cfg(feature = "rand")]
  pub fn rand() -> Self {
    Self::new(random())
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
