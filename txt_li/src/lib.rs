use anyhow::Result;

pub mod restore;
pub use restore::Restore;

#[derive(Default, Debug)]
pub struct TxtLi {
  pub li: Vec<String>,
  pub restore: Restore,
}

pub type ChunkLi = Vec<Vec<String>>;

impl TxtLi {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn push_tran(&mut self, txt: impl Into<String>) {
    self.li.push(txt.into());
  }

  pub fn push_no_tran(&mut self, txt: impl Into<String>) {
    self.restore.push(self.li.len(), txt.into());
  }

  pub fn chunk(self, limit: usize) -> ChunkLi {
    let mut r = vec![];
    let mut t = vec![];
    let mut len = 0;
    for i in self.li {
      let diff = 1 + i.len();
      if len + diff > limit {
        r.push(t);
        if diff < limit {
          t = vec![i];
          len = diff;
        } else {
          let mut end = limit;
          while !i.is_char_boundary(end) && end > 0 {
            end -= 1;
          }
          if end > 0 {
            r.push(vec![i[..end].into()]);
          }
          t = vec![];
          len = 0;
        }
      }
    }

    if !t.is_empty() {
      r.push(t);
    }

    r
  }
}

pub trait Parser {
  fn parse(&self, txt: impl AsRef<str>) -> Result<TxtLi>;
}
