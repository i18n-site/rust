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
    let len = self.li.len();
    self.restore.push(len, txt.into());
    self.restore.push(len, "\n".into());
  }
}

pub trait Parser {
  fn parse(&self, txt: impl AsRef<str>) -> Result<TxtLi>;
}
