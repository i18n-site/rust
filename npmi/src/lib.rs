use std::path::{Path, PathBuf};

use aok::{Null, Result, OK};
use futures::{stream::FuturesUnordered, StreamExt};
use tracing::info;

pub const NODE_MODULES: &str = "node_modules";

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

    #[allow(clippy::never_loop)]
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

pub async fn pkg_json_ver(dir: impl AsRef<Path>) -> Result<Option<String>> {
  let dir = dir.as_ref();
  let pkg_json = dir.join("package.json");
  if let Ok(meta) = tokio::fs::metadata(&pkg_json).await {
    if meta.is_file() {
      if let Ok(bin) = xerr::ok!(tokio::fs::read(pkg_json).await) {
        if let Ok::<npmv::Info, _>(info) = xerr::ok!(sonic_rs::from_slice(&bin)) {
          return Ok(Some(info.version));
        }
      }
    }
  }

  Ok(None)
}

pub async fn is_same_ver(ver: &str, dir: impl AsRef<Path>) -> Result<bool> {
  if let Some(v) = pkg_json_ver(dir).await? {
    return Ok(v == ver);
  }
  Ok(false)
}

impl Npm {
  pub fn new(dir: impl Into<PathBuf>) -> Self {
    Self {
      dir: dir.into().join(NODE_MODULES),
    }
  }

  pub async fn i(&self, name_ver: impl AsRef<str>) -> Null {
    let pkg = Pkg::new(name_ver);
    let out = self.dir.join(&pkg.name);

    if let Ok(Some(ver)) = xerr::ok!(pkg_json_ver(&out).await) {
      if pkg.ver.is_none() {
        return OK;
      }
      if let Some(ref v) = pkg.ver {
        if *v == ver {
          return OK;
        }
      }
    }

    let ver = pkg.ver().await?;
    info!("install {}@{}", pkg.name, ver);
    npmv::tgz(pkg.name, &ver, out).await?;

    OK
  }

  pub async fn u(&self, name_ver: impl AsRef<str>) -> Null {
    let pkg = Pkg::new(name_ver);
    let ver = pkg.ver().await?;
    let out = self.dir.join(&pkg.name);

    if !is_same_ver(&ver, &out).await? {
      info!("upgrade {}@{}", pkg.name, ver);
      npmv::tgz(pkg.name, &ver, out).await?;
    }

    OK
  }
}

macro_rules! func {
  ($dir:expr, $name_ver_li:expr, $func:ident) => {{
    let mut ing = FuturesUnordered::new();
    let dir = $dir.into();
    for name_ver in $name_ver_li {
      let dir = dir.clone();
      ing.push(async move {
        let npm = Npm::new(dir);
        let name_ver = name_ver.as_ref();
        if let Err(err) = npm.$func(name_ver).await {
          tracing::error!("{name_ver} {}", err);
        }
      });
    }

    while let Some(_) = ing.next().await {}

    Ok(())
  }};
}

pub async fn auto<S: AsRef<str>>(dir: impl Into<PathBuf>, name_ver_li: &[S]) -> Null {
  let dir = dir.into();
  let utime = dir.join(NODE_MODULES).join("pkg.utime");
  let now = sts::sec();

  if let Ok(meta) = tokio::fs::metadata(&utime).await {
    if meta.is_file() {
      if let Ok(utime) = xerr::ok!(tokio::fs::read_to_string(&utime).await) {
        if let Ok(utime) = xerr::ok!(utime.parse::<u64>()) {
          if (utime + 7 * 86400) > now {
            i(dir, name_ver_li).await?;
            return OK;
          }
        }
      }
      tokio::fs::remove_file(&utime).await?;
    } else if meta.is_dir() {
      tokio::fs::remove_dir_all(&dir).await?;
    }
  }

  u(dir, name_ver_li).await?;
  tokio::fs::write(utime, format!("{now}")).await?;
  OK
}

pub async fn i<S: AsRef<str>>(dir: impl Into<PathBuf>, name_ver_li: &[S]) -> Null {
  func!(dir, name_ver_li, i)
}

pub async fn u<S: AsRef<str>>(dir: impl Into<PathBuf>, name_ver_li: &[S]) -> Null {
  func!(dir, name_ver_li, u)
}
