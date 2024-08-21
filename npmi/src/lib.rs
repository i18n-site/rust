#![feature(let_chains)]

use std::{
  fmt::Display,
  path::{Path, PathBuf},
};

use aok::{Null, Result, OK};
use dashmap::DashMap;
use futures::stream::{FuturesUnordered, StreamExt};
use serde::Deserialize;
use sver::Ver;
use tracing::error;

pub const PACKAGE_JSON: &str = "package.json";

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
    let name: String;
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

    let name = if name.contains('/') && !name.starts_with('@') {
      "@".to_owned() + &name
    } else {
      name
    };

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

#[derive(Deserialize, Debug)]
pub struct PkgVer {
  pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct PkgDep {
  pub dependencies: Option<std::collections::HashMap<String, String>>,
}

pub async fn load<T: serde::de::DeserializeOwned>(dir: impl AsRef<Path>) -> Result<Option<T>> {
  let dir = dir.as_ref();
  let pkg_json = dir.join(PACKAGE_JSON);
  if let Ok(meta) = tokio::fs::metadata(&pkg_json).await {
    if meta.is_file() {
      if let Ok(bin) = xerr::ok!(tokio::fs::read(pkg_json).await) {
        if let Ok(r) = xerr::ok!(sonic_rs::from_slice(&bin)) {
          return Ok(Some(r));
        }
      }
    }
  }
  Ok(None)
}

impl PkgVer {
  pub async fn load(dir: impl AsRef<Path>) -> Result<Option<Self>> {
    load(dir).await
  }
}

pub struct Npm {
  pub dir: PathBuf,
}

impl Npm {
  pub fn new(dir: impl Into<PathBuf>) -> Self {
    Self { dir: dir.into() }
  }

  pub async fn i(&self, pkg: &Pkg, exist: &DashMap<String, Ver>) -> Null {
    let out = self.dir.join(&pkg.name);

    if let Some(pkg_json) = PkgVer::load(&out).await? {
      if pkg.ver.is_none() {
        return OK;
      }
      if let Some(ref v) = pkg.ver {
        if *v == pkg_json.version {
          return OK;
        }
      }
    }

    let ver = pkg.ver().await?;
    println!("install {}@{}", pkg.name, ver);
    tgz(&pkg.name, &ver, out, exist).await?;

    OK
  }

  pub async fn u(&self, pkg: &Pkg, exist: &DashMap<String, Ver>) -> Null {
    let ver = pkg.ver().await?;
    let out = self.dir.join(&pkg.name);

    let action = if let Some(pkg_json) = PkgVer::load(&out).await? {
      if pkg_json.version == ver {
        return OK;
      }
      "upgrade"
    } else {
      "install"
    };
    println!("{action} {}@{}", pkg.name, ver);
    tgz(&pkg.name, &ver, out, exist).await?;

    OK
  }
}

pub async fn install_dep(out: &Path, name: &str, exist: &DashMap<String, Ver>) -> Null {
  let root = out.display().to_string();
  let strip = name.len() + 1;
  let root_len = root.len();
  if root_len > strip {
    if let Some::<PkgDep>(pkg) = load(out).await? {
      let root: PathBuf = root[..root_len - strip].into();
      if let Some(dep) = pkg.dependencies.as_ref() {
        let mut ing = FuturesUnordered::new();
        for (pkg, ver) in dep {
          let mut pos = 0;
          for (p, i) in ver.char_indices() {
            pos = p;
            if i.is_ascii_digit() {
              break;
            }
          }
          ing.push(_tgz(pkg, &ver[pos..], root.join(pkg), exist));
        }
        while let Some(r) = ing.next().await {
          xerr::log!(r)
        }
      }
    }
  }
  OK
}

pub async fn _tgz(
  name: impl AsRef<str>,
  semver: impl AsRef<str>,
  out: impl AsRef<Path>,
  exist: &DashMap<String, Ver>,
) -> Null {
  let semver = semver.as_ref();
  let name = name.as_ref();
  let out = out.as_ref();
  let ver: Ver = semver.into();

  {
    if let Some(exist_ver) = exist.get(name) {
      if ver <= *exist_ver {
        return OK;
      }
    }
    exist.insert(name.into(), ver.clone());
  }

  if let Some(pkg_json) = PkgVer::load(out).await? {
    let pkg_ver: Ver = pkg_json.version.into();
    if ver <= pkg_ver {
      return OK;
    }
  }

  println!("install {name}@{semver}");
  npmv::tgz(name, semver, out).await?;
  Box::pin(install_dep(out, name, exist)).await?;
  Ok(())
}

pub async fn tgz(
  name: impl AsRef<str>,
  ver: impl AsRef<str>,
  out: impl AsRef<Path>,
  exist: &DashMap<String, Ver>,
) -> Null {
  let name = name.as_ref();
  let out = out.as_ref();
  npmv::tgz(name, ver, out).await?;
  install_dep(out, name, exist).await?;
  OK
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
  if !pkg_li.is_empty() {
    tokio::fs::write(utime, format!("{now}")).await?;
  }
  OK
}

macro_rules! func {
  ($dir:expr, $pkg_li:expr, $func:ident) => {{
    let dir = $dir.into();
    let exist: DashMap<String, Ver> = DashMap::new();
    let exist = &exist;
    {
      let mut ing = FuturesUnordered::new();
      for pkg in $pkg_li {
        let dir = dir.clone();
        ing.push(async move {
          let npm = Npm::new(dir);
          if let Err(err) = npm.$func(pkg, exist).await {
            error!("{pkg} {err}");
          }
        });
      }
      while let Some(_) = ing.next().await {}
    }
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
          Some(fp.strip_prefix(&self.dir).unwrap().to_path_buf())
        } else {
          None
        }
      })
      .collect()
  }

  pub fn new(dir: impl Into<PathBuf>, name_ver_li: &[impl AsRef<str>]) -> Self {
    let dir = dir.into();
    Self {
      dir,
      li: name_ver_li
        .iter()
        .map(|name_ver| Pkg::new(name_ver))
        .collect(),
    }
  }
}
