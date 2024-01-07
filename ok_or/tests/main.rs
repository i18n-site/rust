use std::io::Read;

use ok_or::ok_or;

pub fn size(path: &str) -> usize {
  ok_or!(
    {
      let mut s = String::new();
      std::fs::File::open(path)?.read_to_string(&mut s)?;
      s.len()
    },
    0
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let result = size("not exist");
    assert_eq!(result, 0);
  }
}
