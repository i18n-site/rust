use std::path::PathBuf;

use aok::{Null, OK};

use super::Ckv;
pub struct Fs {
  pub out: PathBuf,
}

impl Ckv for Fs {
  async fn put(&self, rel: impl AsRef<str>, bin: impl AsRef<[u8]>) -> Null {
    ifs::wbin(self.out.join(rel.as_ref()), bin.as_ref())?;
    OK
  }
}

impl Fs {
  pub fn new(out: impl Into<PathBuf>) -> Self {
    Self { out: out.into() }
  }
}
