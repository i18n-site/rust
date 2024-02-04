#[derive(thiserror::Error, Debug)]
pub enum TxtPosError {
  #[error("pos_li.len() != iter len")]
  PosLiLen,
}

#[derive(Debug, Clone, Default)]
pub struct TxtPos<'a> {
  pub txt_li: Vec<&'a str>,
  pub pos_li: Vec<usize>,
}

impl<'a> TxtPos<'a> {
  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      txt_li: Vec::with_capacity(capacity),
      pos_li: Vec::with_capacity(capacity),
    }
  }

  pub fn push(&mut self, txt: &'a str) {
    if !txt.is_empty() {
      self.txt_li.push(txt);
    }
  }

  pub fn push_pos(&mut self, txt: &'a str) {
    if !txt.is_empty() {
      self.pos_li.push(self.txt_li.len());
      self.txt_li.push(txt);
    }
  }

  pub fn push_txt(&mut self, txt: &'a str) {
    crate::trim::push(txt, &mut self.txt_li, &mut self.pos_li);
  }

  pub fn push_txt_line(&mut self, txt: &'a str) {
    crate::trim::push_line(txt, &mut self.txt_li, &mut self.pos_li);
  }

  pub fn merge(
    &self,
    li: impl IntoIterator<Item = impl AsRef<str>>,
  ) -> std::result::Result<String, TxtPosError> {
    let mut li = li.into_iter();
    let mut pos_li = self.pos_li.clone();
    pos_li.reverse();
    let mut pos = pos_li.pop();
    let mut r = String::new();
    for (p, i) in self.txt_li.iter().enumerate() {
      if Some(p) == pos {
        if let Some(s) = li.next() {
          r.push_str(s.as_ref());
        } else {
          return Err(TxtPosError::PosLiLen);
        }
        pos = pos_li.pop();
      } else {
        r.push_str(i);
      }
    }
    Ok(r)
  }
}
