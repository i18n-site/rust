#![feature(let_chains)]
use std::{
  collections::{HashMap, HashSet},
  fs::File,
  io::{self, BufRead as _, BufReader, Read},
  path::PathBuf,
};

mod trie;
pub use trie::Trie;
mod prefix_li;
use aok::Result;
pub use prefix_li::PrefixLi;
use ver_incr::ver_incr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("EXISTS: {0}")]
  FileExists(String),
}

#[derive(Debug)]
pub struct VerFs {
  pub root: PathBuf,
  pub out: PathBuf,
  pub verdir: PathBuf,
  pub log: PathBuf,
  pub hash_ver: HashMap<Box<[u8]>, Box<str>>,
  pub ver: Box<str>,
  pub new_hash: HashSet<Box<[u8]>>,
  // 被添加的文件
  pub rel_ver: HashMap<String, Box<str>>,
}

pub fn latest_ver(
  log: &PathBuf,
  hash_ver: &mut HashMap<Box<[u8]>, Box<str>>,
) -> Result<Box<str>, std::io::Error> {
  let mut latest: Option<Box<str>> = None;
  if log.exists() {
    let mut ver = None;
    let reader = BufReader::new(File::open(log)?);
    for line in reader.lines().map_while(Result::ok) {
      let line = line.trim();
      if line.starts_with("#") || line.is_empty() {
        continue;
      }
      if let Some(v) = line.strip_prefix("@") {
        ver = Some(Box::<str>::from(v));
        if latest.is_none() {
          latest = Some(ver_incr(v).into());
        }
      } else if let Some(ref ver) = ver {
        if let Ok(bin) = burl::d(line) {
          hash_ver.entry(bin.into()).or_insert_with(|| ver.clone());
        }
      }
    }
  }
  Ok(latest.unwrap_or_else(|| "0.1.0".into()))
}

impl VerFs {
  pub fn has_new(&self) -> bool {
    !self.new_hash.is_empty()
  }

  pub fn sorted_rel_ver(&self) -> Vec<(String, Box<str>)> {
    let mut li: Vec<_> = self
      .rel_ver
      .iter()
      .map(|(k, v)| (k.clone(), v.clone()))
      .collect();
    li.sort();
    li
  }

  pub fn throw_if_exists(&self, rel: &str) -> Result<(), Error> {
    if self.rel_ver.contains_key(rel) {
      return Err(Error::FileExists(rel.into()));
    }
    Ok(())
  }

  pub fn wstr(&mut self, rel: impl Into<String>, bin: impl AsRef<str>) -> Result<Box<str>> {
    self.wbin(rel, bin.as_ref().as_bytes())
  }

  pub fn wbin(&mut self, rel: impl Into<String>, bin: impl AsRef<[u8]>) -> Result<Box<str>> {
    let rel = rel.into();
    self.throw_if_exists(&rel)?;
    let bin = bin.as_ref();

    let mut hasher = blake3::Hasher::new();
    let bytes = rel.as_bytes();
    hasher.update(bytes);
    hasher.update(&[0]);
    hasher.update(bin);
    let len = bytes.len() + bin.len();

    let hash: Box<[u8]> = [
      hasher.finalize().as_bytes(),
      &intbin::u64_bin(len as u64)[..],
    ]
    .concat()
    .into();

    let ver = match self.hash_ver.get(&hash) {
      Some(ver) => ver,
      None => {
        let ver = self.ver.clone();
        // println!("new wbin {rel}");
        self.new_hash.insert(hash.clone());
        self.hash_ver.insert(hash, ver);
        ifs::wbin(self.verdir.join(&rel), bin)?;
        &self.ver
      }
    };
    self.rel_ver.insert(rel, ver.clone());
    Ok(ver.clone())
  }

  pub fn cp(&mut self, from: impl Into<String>, to: impl Into<String>) -> Result<Box<str>> {
    let from = from.into();
    let to = to.into();
    self.throw_if_exists(&to)?;
    let fp = self.root.join(&from);

    let mut hasher = blake3::Hasher::new();
    let bytes = to.as_bytes();
    hasher.update(bytes);
    hasher.update(&[0]);

    let mut file = File::open(&fp)?;
    let mut len = bytes.len();
    let mut buffer = [0; 4096];
    loop {
      let n = file.read(&mut buffer)?;
      if n == 0 {
        break;
      }
      hasher.update(&buffer[..n]);
      len += n;
    }
    let hash: Box<[u8]> = [
      hasher.finalize().as_bytes(),
      &intbin::u64_bin(len as u64)[..],
    ]
    .concat()
    .into();

    let ver = match self.hash_ver.get(&hash) {
      Some(ver) => ver,
      None => {
        let ver = self.ver.clone();
        // println!("{from} -> {to}");
        self.new_hash.insert(hash.clone());
        self.hash_ver.insert(hash, ver);
        ifs::cp_rel(&self.root, &from, &self.verdir, &to)?;
        &self.ver
      }
    };
    self.rel_ver.insert(to, ver.clone());
    Ok(ver.clone())
  }

  pub fn load(
    root: impl Into<PathBuf>,
    out: impl Into<PathBuf>,
    log: impl Into<PathBuf>,
  ) -> std::io::Result<Self> {
    let root = root.into();
    let out = out.into();
    let log = log.into();
    let mut hash_ver = HashMap::new();

    let ver = latest_ver(&log, &mut hash_ver)?;

    let verdir = out.join(&*ver);
    Ok(Self {
      root,
      out,
      log,
      ver,
      verdir,
      hash_ver,
      rel_ver: HashMap::new(),
      new_hash: HashSet::new(),
    })
  }

  pub fn save(&self) -> io::Result<bool> {
    let has_new = !self.new_hash.is_empty();
    if has_new {
      let mut li = Vec::with_capacity(self.new_hash.len() + 1);
      li.push(format!("@{}", self.ver));
      for i in &self.new_hash {
        li.push(burl::e(i));
      }
      let new_txt = li.join("\n");
      if self.log.exists() {
        if let Ok(txt) = xerr::ok!(ifs::rtxt(&self.log)) {
          ifs::wtxt(&self.log, format!("{new_txt}\n{txt}"))?;
        }
      } else {
        ifs::wtxt(&self.log, new_txt)?;
      }
    }
    Ok(has_new)
  }
}
