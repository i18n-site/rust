use std::path::PathBuf;

use aok::Result;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("{0} : {1}")]
  Conf(PathBuf, serde_yaml::Error),
}

pub fn load<Conf: serde::de::DeserializeOwned>(fp: &std::path::Path) -> Result<Conf> {
  let conf = ifs::r(fp)?;
  match serde_yaml::from_slice(&conf) {
    Ok(conf) => Ok(conf),
    Err(err) => Err(Error::Conf(fp.to_path_buf(), err).into()),
  }
}

pub fn load_or_exit<Conf: serde::de::DeserializeOwned>(dir: &std::path::Path) -> Conf {
  match load(dir) {
    Ok(conf) => conf,
    Err(err) => {
      eprintln!("❌ {err}");
      std::process::exit(1);
    }
  }
}
