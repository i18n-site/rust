use std::path::Path;

use aok::{Null, Result, OK};
use gxhash::HashMap;
use serde::{Deserialize, Serialize};

use super::Seo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
  pub endpoint: String,
  pub region: Option<String>,
  pub ak: String,
  pub sk: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct I18nConf {
  pub s3: Option<HashMap<String, Vec<Conf>>>,
}

pub struct S3 {}

impl Seo for S3 {
  fn init(root: &Path, name: &str, host: &str) -> Result<Self> {
    // let out = root.join("out").join(name).join("htm");
    let conf: I18nConf = i18::env::load()?;
    dbg!(conf);

    Ok(Self {})
  }

  async fn put(&self, rel: impl AsRef<str>, bin: impl AsRef<[u8]>) -> Null {
    // ifs::wbin(&self.out.join(rel.as_ref()), bin.as_ref())?;
    OK
  }
}
