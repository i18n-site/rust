pub struct CIter<'a, T> {
  idx: usize,
  pub li: &'a [T],
  ed: usize,
}

impl<'a, T> CIter<'a, T> {
  pub fn pos(&self) -> usize {
    if self.idx == 0 { 0 } else { self.idx - 1 }
  }

  pub fn new(li: &'a [T], pos: usize) -> Self {
    CIter {
      li,
      idx: pos,
      ed: 0,
    }
  }

  #[cfg(feature = "rand")]
  pub fn rand(li: &'a [T]) -> Self {
    use rand::Rng;
    let mut rng = rand::rng();
    let n: usize = rng.random_range(0..li.len());
    Self::new(li, n)
  }
}

impl<'a, T> Iterator for CIter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    let len = self.li.len();
    if self.ed < len {
      let idx = self.idx % len;
      let r = Some(&self.li[idx]);
      self.ed += 1;
      self.idx += 1;
      r
    } else {
      None
    }
  }
}
