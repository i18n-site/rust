pub fn trim_start(s: impl AsRef<str>, trimer: impl Fn(char) -> bool) -> usize {
  let s = s.as_ref();
  for (p, i) in s.char_indices() {
    if !trimer(i) {
      return p;
    }
  }
  s.len()
}

pub fn trim_end(s: impl AsRef<str>, trimer: impl Fn(char) -> bool) -> usize {
  let s = s.as_ref();
  let mut p = 0;
  for (pos, i) in s.char_indices().rev() {
    if !trimer(i) {
      break;
    }
    p = pos;
  }
  p
}

pub fn chars(s: &str) -> impl Iterator<Item = (usize, char)> + '_ {
  let mut iter = s.char_indices();

  let mut pre = iter.next();
  std::iter::from_fn(move || {
    if let Some(p) = pre {
      let t = iter.next();
      let r;
      if let Some((pos, _)) = t {
        r = (pos, p.1);
        pre = t;
      } else {
        r = (s.len(), p.1);
        pre = None;
      }
      Some(r)
    } else {
      None
    }
  })
}
