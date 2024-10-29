pub struct PosLines<'a> {
  txt: &'a str,
  pos: usize,
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
    if self.pos == txt_len {
      return None;
    }

    let sub = &self.txt[self.pos..];
    let mut chars = sub.char_indices();
    let start;

    'out: loop {
      while let Some((i, c)) = chars.next() {
        if c == '\n' || c == '\r' {
          continue;
        }
        start = i;
        break 'out;
      }
      start = sub.len();
      break;
    }

    while let Some((i, c)) = chars.next() {
      if c == '\n' || c == '\r' {
        let begin = self.pos + start;
        self.pos = self.pos + i;
        return Some((begin, &self.txt[begin..self.pos]));
      }
    }

    if start < sub.len() {
      let begin = self.pos + start;
      self.pos = txt_len;
      return Some((begin, &self.txt[begin..]));
    }

    return None;
  }
}