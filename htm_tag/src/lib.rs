pub fn htm_tag(s: &str) -> Option<&str> {
  let mut iter = s.char_indices();
  if let Some((pos, c)) = iter.next()
    && c.is_ascii_alphabetic()
  {
    let begin = pos;
    let mut end = s.len();
    for (pos, c) in iter {
      if !c.is_alphanumeric() {
        end = pos;
        break;
      }
    }
    return Some(&s[begin..end]);
  }
  None
}
