pub struct CIter<'a, T> {
  idx: usize,
  li: &'a [T],
  ed: usize,
}

impl<'a, T> CIter<'a, T> {
  pub fn pos(&self) -> usize {
    let len = self.li.len();
    if self.idx == 0 {
      0
    } else {
      self.idx - 1
    }
  }

  pub fn new(li: &'a [T], pos: usize) -> Self {
    CIter {
      li,
      idx: pos,
      ed: 0,
    }
  }
}

impl<'a, T> Iterator for CIter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    let len = self.li.len();
    if self.ed < len {
      let idx = self.idx % len;
      self.ed += 1;
      self.idx += 1;
      Some(&self.li[idx])
    } else {
      None
    }
  }
}
