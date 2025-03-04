#[derive(Debug, Clone)]
pub struct PosLines<'a> {
  pub txt: &'a str,
  pub pos: usize,
}

impl<'a> PosLines<'a> {
  pub fn new(txt: &'a str) -> Self {
    Self { txt, pos: 0 }
  }
}

impl<'a> Iterator for PosLines<'a> {
  type Item = (usize, &'a str);

  fn next(&mut self) -> Option<Self::Item> {
    let txt_len = self.txt.len();
    if self.pos >= txt_len {
      return None;
    }

    let sub = &self.txt[self.pos..];
    let mut chars = sub.char_indices();
    let start;

    #[allow(clippy::never_loop)]
    'out: loop {
      for (i, c) in chars.by_ref() {
        if c == '\n' || c == '\r' {
          continue;
        }
        start = i;
        break 'out;
      }
      start = sub.len();
      break;
    }

    for (i, c) in chars {
      if c == '\n' || c == '\r' {
        let begin = self.pos + start;
        self.pos += i;
        return Some((begin, &self.txt[begin..self.pos]));
      }
    }

    if start < sub.len() {
      let begin = self.pos + start;
      self.pos = txt_len;
      return Some((begin, &self.txt[begin..]));
    }

    None
  }
}
