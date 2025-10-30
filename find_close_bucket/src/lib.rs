pub fn find_close_bucket(txt: impl AsRef<str>, start: char, end: char) -> Option<usize> {
  let txt = txt.as_ref();

  let mut count: usize = 0;
  let mut iter = txt.char_indices();

  while let Some((i, c)) = iter.next() {
    if c == start {
      count += 1;
    } else if c == end {
      if count == 0 {
        return Some(i);
      }
      count -= 1;
    } else if c == '\\' {
      iter.next();
    }
  }

  None
}
