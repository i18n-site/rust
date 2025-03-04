use roaring::RoaringTreemap;

use crate::TxtPos;

impl<'a> TxtPos<'a> {
  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      txt_li: Vec::with_capacity(capacity),
      pos_li: RoaringTreemap::new(),
    }
  }

  pub fn push(&mut self, txt: &'a str) {
    if !txt.is_empty() {
      self.txt_li.push(txt);
    }
  }

  pub fn push_pos(&mut self, txt: &'a str) {
    if !txt.is_empty() {
      self.pos_li.push(self.txt_li.len() as u64);
      self.txt_li.push(txt);
    }
  }

  pub fn push_txt(&mut self, txt: &'a str) {
    crate::trim::push(txt, &mut self.txt_li, &mut self.pos_li);
  }

  pub fn push_txt_line(&mut self, txt: &'a str) {
    crate::trim::push_line(txt, &mut self.txt_li, &mut self.pos_li);
  }
}
