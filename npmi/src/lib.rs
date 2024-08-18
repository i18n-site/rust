use std::path::PathBuf;

use aok::{Null, Result, OK};

#[derive(Debug)]
pub struct Pkg {
  pub name: String,
  pub ver: Option<String>,
}

impl Pkg {
  pub fn new(name_ver: impl AsRef<str>) -> Self {
    let name_ver = name_ver.as_ref();
    let name;
    let ver;
    loop {
      if name_ver.len() > 1 {
        if let Some(mut p) = name_ver[1..].find('@') {
          p += 1;
          name = name_ver[..p].into();
          ver = Some(name_ver[p + 1..].into());
          break;
        }
      }

      name = name_ver.into();
      ver = None;
      break;
    }

    Self { name, ver }
  }

  async fn ver(&self) -> Result<String> {
    if let Some(ver) = &self.ver {
      Ok(ver.clone())
    } else {
      npmv::latest(&self.name).await
    }
  }
}

pub struct Npm {
  pub dir: PathBuf,
}

impl Npm {
  pub fn new(dir: impl Into<PathBuf>) -> Self {
    Self {
      dir: dir.into().join("node_modules"),
    }
  }

  pub async fn i(name_ver: impl AsRef<str>) -> Null {
    let pkg = Pkg::new(name_ver);
    let ver = pkg.ver().await?;
    // pkg.tgz(&ver, &pkg.dir).await;

    OK
  }
}
