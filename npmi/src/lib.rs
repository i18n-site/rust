use std::{
  fmt::Display,
  path::{Path, PathBuf},
};

use aok::{Null, Result, OK};
use futures::stream::{FuturesUnordered, StreamExt};
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct Pkg {
  pub name: String,
  pub ver: Option<String>,
}

impl Display for Pkg {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)?;
    if let Some(ver) = &self.ver {
      write!(f, "@{}", ver)?;
    }
    Ok(())
  }
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

pub struct Npm {
  pub dir: PathBuf,
}

impl Npm {
  pub fn new(dir: impl Into<PathBuf>) -> Self {
    Self { dir: dir.into() }
  }

  pub async fn i(&self, pkg: &Pkg) -> Null {
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
    npmv::tgz(&pkg.name, &ver, out).await?;

    OK
  }

  pub async fn u(&self, pkg: &Pkg) -> Null {
    let ver = pkg.ver().await?;
    let out = self.dir.join(&pkg.name);

    if !is_same_ver(&ver, &out).await? {
      info!("upgrade {}@{}", pkg.name, ver);
      npmv::tgz(&pkg.name, &ver, out).await?;
    }

    OK
  }
}

pub async fn auto(dir: impl Into<PathBuf>, pkg_li: &[Pkg]) -> Null {
  let dir = dir.into();
  let utime = dir.join("pkg.utime");
  let now = sts::sec();

  if let Ok(meta) = tokio::fs::metadata(&utime).await {
    if meta.is_file() {
      if let Ok(utime) = xerr::ok!(tokio::fs::read_to_string(&utime).await) {
        if let Ok(utime) = xerr::ok!(utime.parse::<u64>()) {
          if (utime + 7 * 86400) > now {
            i(dir, pkg_li).await?;
            return OK;
          }
        }
      }
      tokio::fs::remove_file(&utime).await?;
    } else if meta.is_dir() {
      tokio::fs::remove_dir_all(&dir).await?;
    }
  }

  u(dir, pkg_li).await?;
  tokio::fs::write(utime, format!("{now}")).await?;
  OK
}

macro_rules! func {
  ($dir:expr, $pkg_li:expr, $func:ident) => {{
    let mut ing = FuturesUnordered::new();
    let dir = $dir.into();
    for pkg in $pkg_li {
      let dir = dir.clone();
      ing.push(async move {
        let npm = Npm::new(dir);
        if let Err(err) = npm.$func(pkg).await {
          tracing::error!("{pkg} {err}");
        }
      });
    }

    while let Some(_) = ing.next().await {}

    Ok(())
  }};
}

pub async fn i(dir: impl Into<PathBuf>, pkg_li: &[Pkg]) -> Null {
  func!(dir, pkg_li, i)
}

pub async fn u(dir: impl Into<PathBuf>, pkg_li: &[Pkg]) -> Null {
  func!(dir, pkg_li, u)
}

pub struct PkgLi {
  pub li: Vec<Pkg>,
  pub dir: PathBuf,
}

pub const NODE_MODULES: &str = "node_modules";

impl PkgLi {
  pub async fn auto(&self) -> Null {
    auto(&self.dir, &self.li).await
  }

  pub async fn u(&self) -> Null {
    u(&self.dir, &self.li).await
  }

  pub fn rel_li(&self, rel: impl AsRef<str>) -> Vec<PathBuf> {
    self
      .li
      .iter()
      .filter_map(|pkg| {
        let fp = self.dir.join(&pkg.name).join(rel.as_ref());
        if fp.exists() {
          Some(fp)
        } else {
          None
        }
      })
      .collect()
  }

  pub fn new(dir: impl Into<PathBuf>, name_ver_li: &[impl AsRef<str>]) -> Self {
    let dir = dir.into();
    Self {
      dir: dir.join(NODE_MODULES),
      li: name_ver_li
        .iter()
        .map(|name_ver| Pkg::new(name_ver))
        .collect(),
    }
  }
}
