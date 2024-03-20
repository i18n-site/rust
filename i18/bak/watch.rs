use std::{
  collections::{HashMap, HashSet},
  fs,
  io::Write,
  path::PathBuf,
};

use aok::Result;
use ifs::{b3_len, conf::Item};

use crate::api;

pub const CACHE: &str = "cache";
pub const HASH: &str = "hash";

pub const DIR_LI: &[&str] = &[CACHE, HASH];

#[derive(Clone, PartialEq, Debug)]
pub struct Txt {
  pub v: String,
  pub src: Option<Box<[u8]>>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Change {
  True(Txt),
  False,
  FileNotExist,
}

#[derive(Clone, Default)]
pub struct Watch {
  dir: PathBuf,
  gen: PathBuf,
  cache: HashMap<String, Change>,
  exist: HashSet<String>,
}

impl Watch {
  pub fn new(dir: impl Into<PathBuf>) -> Self {
    let dir = dir.into();
    let gen = dir.join(".gen");

    #[allow(clippy::never_loop)]
    loop {
      if let Ok(meta) = fs::metadata(&gen) {
        if meta.is_dir() {
          break;
        }
        xerr::log!(fs::remove_file(&gen));
      }
      fs::create_dir_all(&gen).unwrap();
      if let Ok(mut f) = ifs::w(gen.join(".gitignore")) {
        xerr::log!(f.write_all(br#"/cache/"#));
        xerr::log!(f.flush());
      }
      break;
    }

    Self {
      dir,
      gen,
      ..Default::default()
    }
  }

  fn meta(&mut self, rel: &str) -> Item<api::Meta> {
    Item::<api::Meta>::new(self.gen.join(CACHE), rel)
  }

  fn hash(&mut self, rel: &str) -> Item<api::Hash> {
    Item::<api::Hash>::new(self.gen.join(HASH), rel)
  }

  pub fn save(
    &mut self,
    rel: impl AsRef<str>,
    txt: impl AsRef<str>,
    src: impl AsRef<[u8]>,
  ) -> Result<Box<[u8]>> {
    let rel = rel.as_ref();
    let txt = txt.as_ref().as_bytes();
    let src = src.as_ref();
    let hash = b3_len(txt);
    let fp = self.dir.join(rel);

    if !src.is_empty() {
      if let Ok(mut f) = xerr::ok!(ifs::w(&fp)) {
        f.write_all(txt)?;
        f.flush()?;
      }
    }

    if let Some((size, mtime)) = ifs::size_mtime(&fp) {
      self.meta(rel).set(api::Meta { size, mtime });
      self.hash(rel).set(api::Hash {
        src: src.as_ref().into(),
        v: hash.clone().into(),
      });
    }

    Ok(hash)
  }

  async fn _is_change(&mut self, rel: &str) -> Change {
    self.exist.insert(rel.into());

    if let Some((size, mtime)) = ifs::size_mtime(self.dir.join(rel)) {
      let pre = self.meta(rel);
      if let Some(p) = pre.get() {
        if p.size == size && p.mtime == mtime {
          return Change::False;
        }
      }

      if let Ok(txt) = xerr::ok!(ifs::r(self.dir.join(rel))) {
        let pre_hash = self.hash(rel);
        let v: String = String::from_utf8_lossy(&txt).into();
        let hash = b3_len(&txt);

        if let Some(p) = pre_hash.get() {
          if *p.v == *hash {
            self.meta(rel).set(api::Meta { size, mtime });
            dbg!((rel, "hash same"));
            return Change::False;
          }
          return Change::True(Txt {
            v,
            src: if p.src.is_empty() {
              None
            } else {
              Some(p.src.into())
            },
          });
        }
        return Change::True(Txt { v, src: None });
      }
    }
    Change::FileNotExist
  }

  pub async fn is_change(&mut self, rel: impl AsRef<str>) -> Change {
    let rel = rel.as_ref();

    let r = self.cache.get(rel);
    if let Some(r) = r {
      return r.clone();
    }

    let r = self._is_change(rel).await;
    self.cache.insert(rel.into(), r.clone());
    r
  }

  pub fn purge(&self) {
    use walkdir::WalkDir;

    let exist = &self.exist;
    for i in DIR_LI {
      let dir = self.gen.join(i);
      let walker = WalkDir::new(&dir);
      for i in walker {
        if let Ok(i) = xerr::ok!(i) {
          if i.file_type().is_file() {
            let path = i.path();
            if let Ok(rel) = xerr::ok!(path.strip_prefix(&dir)) {
              let rel: String = rel.as_os_str().to_string_lossy().into();
              if !exist.contains(&rel) {
                xerr::log!(fs::remove_file(path));
              }
            }
          }
        }
      }
    }
  }
}
