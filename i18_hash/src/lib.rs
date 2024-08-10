#![feature(let_chains)]

use std::{
  ops::{Deref, DerefMut},
  path::{Path, PathBuf},
};

use aok::Result;
use bincode::{Decode, Encode};
use gxhash::{HashMap, HashMapExt};
use lang::{Lang, LANG_CODE};
use tzst::zst;

#[derive(Default, Clone, Debug)]
pub struct LangLi(pub Vec<Lang>);

impl From<LangLi> for Vec<u8> {
  fn from(val: LangLi) -> Self {
    vb::e(val.0.iter().map(|i| *i as u64).collect::<Vec<_>>())
  }
}

impl From<LangLi> for bytes::Bytes {
  fn from(val: LangLi) -> Self {
    let r: Vec<u8> = val.into();
    r.into()
  }
}

impl Deref for LangLi {
  type Target = Vec<Lang>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Decode, Encode, Debug, Clone)]
pub struct Meta {
  pub hash: Vec<u8>,
  pub to_li: Vec<u8>,
}

#[derive(Decode, Encode, Default)]
pub struct LangMeta(HashMap<u16, Meta>);

impl DerefMut for LangMeta {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Deref for LangMeta {
  type Target = HashMap<u16, Meta>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub fn save(fp: impl AsRef<Path>, lang_meta: &LangMeta) -> Result<()> {
  let bin = bce::e(lang_meta)?;
  Ok(ifs::wbin(fp, zst::e(&bin)?)?)
}

pub struct I18Hash {
  pub root: PathBuf,
  pub dir_hash: PathBuf,
  pub cache: HashMap<String, LangMeta>,
}

#[derive(Debug, Clone)]
pub struct File {
  pub lang: u16,
  pub rel: String,
  pub pre_hash: Vec<u8>,
  pub meta: Meta,
  pub len: usize,
}

impl I18Hash {
  pub fn get_lang_meta(&mut self, rel: impl Into<String>) -> &mut LangMeta {
    let rel = rel.into();
    self.cache.entry(rel.clone()).or_insert_with(|| {
      let fp = self.dir_hash.join(&rel);
      if fp.exists()
        && let Ok(cache) = xerr::ok!(ifs::r(&fp))
        && let Ok(cache) = xerr::ok!(zst::d(&cache))
        && let Ok(l) = xerr::ok!(bce::d(&cache))
      {
        l
      } else {
        Default::default()
      }
    })
  }

  pub fn new(root: impl Into<PathBuf>) -> Self {
    let root = root.into();
    Self {
      dir_hash: root.join(".i18n").join("hash"),
      root,
      cache: HashMap::new(),
    }
  }

  pub fn changed(&mut self, rel_li: Vec<(String, LangLi)>) -> Result<Vec<File>> {
    let mut rel_lang = HashMap::<_, Vec<_>>::new();
    for (i, to_li) in rel_li {
      if let Some(p) = i.find("/")
        && p + 1 < i.len()
      {
        let lang = &i[..p];
        if let Some(lang) = LANG_CODE.iter().position(|i| *i == lang) {
          let rel = &i[p + 1..];
          let fp = self.root.join(&i);
          let txt = refmt::fp(&fp)?;
          let bin = txt.as_bytes();
          rel_lang.entry(rel.to_owned()).or_default().push((
            lang,
            xhash::xhash(bin),
            bin.len(),
            to_li,
          ));
        }
      }
    }

    let mut r = vec![];

    let dir_hash = self.dir_hash.clone();
    let root = self.root.clone();

    for (rel, lang_li) in rel_lang.into_iter() {
      let lang_meta_map = self.get_lang_meta(&rel);
      for (lang, hash, len, to_li) in lang_li {
        // 空文件不翻译
        if len == 0 {
          let mut change = lang_meta_map.remove(&(lang as u16)).is_some();
          for i in to_li.0 {
            change |= lang_meta_map.remove(&(i as u16)).is_some();
            let rel = format!("{}/{rel}", LANG_CODE[i as usize]);
            ifs::wbin(root.join(&rel), b"")?;
          }
          if change {
            save(dir_hash.join(&rel), lang_meta_map)?;
          }
          continue;
        }

        let to_li: Vec<u8> = to_li.into();
        let lang = lang as u16;
        let pre_hash = if let Some(pre) = lang_meta_map.get(&lang) {
          if to_li == pre.to_li && hash == pre.hash {
            continue;
          }
          pre.hash.clone()
        } else {
          // 没有pre_hash也没有to, 忽略
          if to_li.is_empty() {
            continue;
          }
          Default::default()
        };

        r.push(File {
          lang,
          rel: rel.clone(),
          pre_hash,
          len,
          meta: Meta { hash, to_li },
        });
      }
    }

    Ok(r)
  }

  pub fn save(
    &mut self,
    rel: impl AsRef<str>,
    lang_meta: impl IntoIterator<Item = (u16, Meta)>,
  ) -> Result<()> {
    let rel = rel.as_ref();
    let fp = self.dir_hash.join(rel);
    let l = self.get_lang_meta(rel);
    for (k, v) in lang_meta {
      l.0.insert(k, v);
    }
    save(fp, l)
  }
}
