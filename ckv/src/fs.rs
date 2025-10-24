use std::{
  io::{BufReader, BufWriter},
  path::PathBuf,
};

use aok::{OK, Void};

use super::Ckv;
pub struct Fs {
  pub out: PathBuf,
}

impl Ckv for Fs {
  async fn put(&self, rel: impl AsRef<str>, bin: impl AsRef<[u8]>) -> Void {
    ifs::wbin(self.out.join(rel.as_ref()), bin.as_ref())?;
    OK
  }

  async fn put_path(&self, rel: impl AsRef<str> + Send, path: &str) -> Void {
    let out_fp = self.out.join(rel.as_ref());
    let mut read = std::fs::File::open(path)?;
    let mut read = BufReader::new(&mut read);

    if let Some(p) = out_fp.parent()
      && !p.exists()
    {
      std::fs::create_dir_all(p)?;
    }
    let mut out = std::fs::File::create(&out_fp)?;
    let mut out = BufWriter::new(&mut out);
    std::io::copy(&mut read, &mut out).unwrap();
    OK
  }
}

impl Fs {
  pub fn new(out: impl Into<PathBuf>) -> Self {
    Self { out: out.into() }
  }
}
