#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

use anyhow::Result;

pub mod restore;
pub use restore::Restore;

#[derive(Default, Debug)]
pub struct TxtLi {
  pub li: Vec<String>,
  pub restore: Restore,
}

impl TxtLi {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn push_tran(&mut self, txt: impl Into<String>) {
    self.li.push(txt.into());
  }

  pub fn push_tran_line(&mut self, txt: impl Into<String>) {
    self.push_tran(txt);
    self.push_no_tran("\n");
  }

  pub fn push_no_tran(&mut self, txt: impl Into<String>) {
    self.restore.push(self.li.len(), txt.into());
  }

  pub fn push_no_tran_line(&mut self, txt: impl Into<String>) {
    self.push_no_tran(txt.into() + "\n");
  }

  #[cfg(feature = "push_trim_line")]
  pub fn push_trim_line(&mut self, txt: impl Into<String>) {
    let txt = txt.into();
    let txt_len = txt.len();
    let mut split_pos = txt_len;

    let mut iter = txt.char_indices().peekable();
    while let Some((pos, i)) = iter.next() {
      if i == '-'
        && let Some((_, next)) = iter.peek()
        && (".-|:".contains(*next) || next.is_whitespace())
      {
        continue;
      }

      if "*_".contains(i)
        && let Some((_, next)) = iter.peek()
        && *next == i
      {
        split_pos = pos;
        break;
      }

      if !("#>.:|=".contains(i) || i.is_whitespace()) {
        split_pos = pos;
        break;
      }
    }
    if split_pos > 0 {
      self.push_no_tran(&txt[..split_pos]);
    }
    if split_pos < txt_len {
      self.push_tran(&txt[split_pos..]);
    }
    self.push_no_tran("\n");
  }
}

pub trait Parser {
  fn parse(&self, txt: impl AsRef<str>) -> Result<TxtLi>;
}
