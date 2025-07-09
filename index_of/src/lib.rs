pub trait IndexOf<T> {
  fn index_of(&self, target: &T) -> Option<usize>;
}

impl<T: PartialEq> IndexOf<T> for &[T] {
  fn index_of(&self, target: &T) -> Option<usize> {
    for (i, x) in self.iter().enumerate() {
      if x == target {
        return Some(i);
      }
    }
    None
  }
}

impl<T: PartialEq, const N: usize> IndexOf<T> for [T; N] {
  fn index_of(&self, target: &T) -> Option<usize> {
    for (i, x) in self.iter().enumerate() {
      if x == target {
        return Some(i);
      }
    }
    None
  }
}
