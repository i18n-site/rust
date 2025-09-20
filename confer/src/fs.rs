use std::{fs, io, path::PathBuf};

use crate::Confer;

pub struct FsConf {
  pub path: PathBuf,
}

impl FsConf {
  pub fn new(path: impl Into<PathBuf>) -> Self {
    Self { path: path.into() }
  }
}

impl FsConf {
  pub fn load(&self) -> io::Result<Confer> {
    let path = &self.path;
    let txt = if path.exists() {
      fs::read_to_string(path)?
    } else {
      String::new()
    };

    Ok(Confer::new(txt))
  }

  pub fn dump(&self, conf: &Confer) -> io::Result<()> {
    if conf.changed {
      fs::write(&self.path, conf.to_string())?
    }
    Ok(())
  }
}
