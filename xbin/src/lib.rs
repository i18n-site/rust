pub fn concat<T: AsRef<[u8]>>(iter: impl IntoIterator<Item = T>) -> Vec<u8> {
  let mut r = Vec::new();
  for i in iter {
    r.extend(i.as_ref());
  }
  r
}

#[macro_export]
macro_rules! concat {
  ($($i:expr),*)=>{
    &[
      $($i.as_ref()),*
    ].concat()[..]
  }
}
