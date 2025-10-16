use rand::Rng;

pub struct RandIter<'a, T> {
  slice: &'a [T],
  pos: usize,
  count: usize,
}

impl<'a, T> RandIter<'a, T> {
  pub fn new(slice: &'a [T]) -> Self {
    let len = slice.len();
    let start = if len > 0 {
      rand::rng().random_range(0..len)
    } else {
      0
    };
    RandIter {
      slice,
      pos: start,
      count: 0,
    }
  }
}

impl<'a, T> Iterator for RandIter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    let len = self.slice.len();
    if self.count == len {
      None
    } else {
      let index = self.pos;
      self.pos = (self.pos + 1) % len;
      self.count += 1;
      Some(&self.slice[index])
    }
  }
}
