#![feature(let_chains)]

use std::{
  collections::{HashMap, HashSet},
  fs::{File, Metadata},
  hash::Hasher,
  io::{BufRead, BufReader, Read},
  path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use aok::{Void, OK};
use set_mtime::set_mtime;
use xxhash_rust::xxh3::Xxh3DefaultBuilder;

pub fn hash(fp: impl AsRef<Path>) -> std::io::Result<u128> {
  let file = File::open(fp)?;
  let mut reader = BufReader::new(file);
  let mut hasher = Xxh3DefaultBuilder.build();

  let mut buffer = [0; 16384];
  loop {
    let bytes_read = reader.read(&mut buffer)?;
    if bytes_read == 0 {
      break;
    }
    hasher.write(&buffer[..bytes_read]);
  }

  Ok(hasher.digest128())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LenTs {
  pub ts: u64,
  pub len: u64,
}

impl LenTs {
  pub fn new(meta: Metadata) -> Self {
    Self {
      ts: meta
        .modified()
        .unwrap()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(),
      len: meta.len(),
    }
  }
}

pub fn path_meta(path: impl AsRef<Path>) -> std::io::Result<Meta> {
  let len_ts = LenTs::new(std::fs::metadata(path.as_ref())?);
  let hash = hash(path.as_ref())?;
  Ok(Meta { len_ts, hash })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
  pub len_ts: LenTs,
  pub hash: u128,
}

#[derive(Debug)]
pub struct Scan {
  pub root: PathBuf,
  pub rel_len_ts: HashMap<String, LenTs>,
}

#[derive(Debug)]
pub struct Diff {
  pub changed: Vec<(String, Meta)>,
  pub no_change: Vec<(String, Meta)>,
  pub db: PathBuf,
  pub has_change: bool,
  pub refresh: HashSet<String>,
  pub root: PathBuf,
}

impl Diff {
  pub fn save(mut self) -> Void {
    let mut li = vec![];
    for (rel, meta) in self.changed.into_iter().chain(self.no_change) {
      let meta = burl::e(pc::e::<Meta>(if self.refresh.remove(&rel) {
        path_meta(self.root.join(&rel))?
      } else {
        meta
      })?);

      li.push(format!("{rel}#{meta}"));
    }

    // 确保没有修改时候,diff不会变化

    li.sort();
    let db = li.join("\n");
    ifs::wstr(&self.db, db)?;

    OK
  }
}

impl Scan {
  pub fn diff(&self, db: impl Into<PathBuf>) -> std::io::Result<Diff> {
    let mut changed = vec![];
    let mut no_change = vec![];
    let db = db.into();
    let mut rel_len_ts = self.rel_len_ts.clone();
    if db.exists() {
      let file = std::io::BufReader::new(std::fs::File::open(&db)?);
      for line in file.lines().map_while(Result::ok) {
        let line = line.trim_end();
        if let Some(i) = line.chars().next() {
          if "<>#".contains(i) {
            continue;
          }

          if let Some(pos) = line.rfind('#') {
            let bin = &line[pos + 1..];
            if let Ok(meta) = burl::d(bin) {
              if let Ok::<Meta, _>(meta) = pc::d(&meta) {
                let rel = &line[..pos];
                if let Some(len_ts) = rel_len_ts.remove(rel) {
                  if len_ts.len == meta.len_ts.len && len_ts.ts == meta.len_ts.ts {
                    no_change.push((rel.into(), meta));
                  } else {
                    let fp = self.root.join(rel);
                    let hash = hash(&fp)?;
                    if hash == meta.hash {
                      set_mtime(&fp, meta.len_ts.ts)?;
                      no_change.push((
                        rel.into(),
                        Meta {
                          len_ts: LenTs {
                            len: len_ts.len,
                            ts: meta.len_ts.ts,
                          },
                          hash,
                        },
                      ));
                    } else {
                      changed.push((rel.into(), Meta { len_ts, hash }));
                    }
                  }
                }
              }
            }
          }
        } else {
          continue;
        }
      }
    }

    for (rel, len_ts) in rel_len_ts.drain() {
      let fp = self.root.join(&rel);
      changed.push((
        rel,
        Meta {
          len_ts,
          hash: hash(fp)?,
        },
      ));
    }

    Ok(Diff {
      has_change: !changed.is_empty() || no_change.len() != self.rel_len_ts.len(),
      changed,
      db,
      no_change,
      refresh: HashSet::new(),
      root: self.root.clone(),
    })
  }

  pub fn add(&mut self, rel: impl AsRef<str>) -> Void {
    let rel = rel.as_ref();
    let fp = self.root.join(rel);
    if let Ok(meta) = std::fs::metadata(fp) {
      self.rel_len_ts.insert(rel.into(), LenTs::new(meta));
    }
    OK
  }

  pub fn new(
    root: impl Into<PathBuf>,
    build: impl Fn(&mut ignore::WalkBuilder) -> &ignore::WalkBuilder,
  ) -> std::io::Result<Self> {
    let root = root.into();
    let mut rel_len_ts = HashMap::default();

    let mut b = ignore::WalkBuilder::new(&root);
    build(&mut b);
    for entry in b.build() {
      if let Ok(entry) = entry
        && let Some(file_type) = entry.file_type()
        && file_type.is_file()
      {
        let path = entry.path();
        if let Ok(meta) = std::fs::metadata(path)
          && let Ok(rel) = path.strip_prefix(&root)
        {
          let rel = rel.to_string_lossy();
          let rel = ifs::unix_path(rel);
          rel_len_ts.insert(rel, LenTs::new(meta));
        }
      }
    }

    Ok(Self { rel_len_ts, root })
  }
}
