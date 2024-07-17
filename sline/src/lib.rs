use std::str::from_utf8_unchecked;

pub struct Line<'a> {
  pub bin: &'a [u8],
  pub pre: usize,
  pub cur: usize,
}

impl<'a> Line<'a> {
  pub fn new(str: &'a str) -> Self {
    Self {
      bin: str.as_bytes(),
      pre: 0,
      cur: 0,
    }
  }
}

impl<'a> Iterator for Line<'a> {
  type Item = &'a str;

  fn next(&mut self) -> Option<Self::Item> {
    let bin = self.bin;
    let len = bin.len();
    let mut cur = self.cur;
    while cur < len {
      self.cur += 1;
      let i = bin[cur];
      let is_break = if i == b'\r' {
        let t = cur + 1;
        if t < len && bin[t] == b'\n' {
          self.cur += 1;
        }
        true
      } else {
        i == b'\n'
      };
      if is_break {
        let r = &bin[self.pre..cur];
        self.pre = self.cur;
        return Some(unsafe { from_utf8_unchecked(r) });
      }
      cur = self.cur;
    }
    if self.cur != self.pre {
      let r = Some(unsafe { from_utf8_unchecked(&bin[self.pre..]) });
      self.pre = self.cur;
      return r;
    }
    None
  }
}
