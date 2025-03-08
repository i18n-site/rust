#![feature(let_chains)]

use std::{
  collections::HashMap,
  fs::File,
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
}

impl Diff {
  pub fn save(&self) -> Void {
    let mut li = vec![];
    for (rel, meta) in self.changed.iter().chain(&self.no_change) {
      let meta = burl::e(pc::e::<Meta>(meta)?);
      li.push(format!("{rel}#{meta}"));
    }

    let db = li.join("\n");
    li.sort();

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
    })
  }

  pub fn add(&mut self, rel: impl AsRef<str>) -> Void {
    let rel = rel.as_ref();
    let fp = self.root.join(rel);
    if let Ok(meta) = std::fs::metadata(fp)
      && let Ok(ts) = meta.modified()
    {
      self.rel_len_ts.insert(
        rel.into(),
        LenTs {
          ts: ts.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
          len: meta.len(),
        },
      );
    }
    OK
  }

  pub fn new(root: impl Into<PathBuf>) -> std::io::Result<Self> {
    let root = root.into();
    let mut rel_len_ts = HashMap::default();

    for entry in ignore::Walk::new(&root).into_iter() {
      if let Ok(entry) = entry
        && let Some(file_type) = entry.file_type()
        && file_type.is_file()
      {
        let path = entry.path();
        if let Ok(meta) = std::fs::metadata(path)
          && let Ok(ts) = meta.modified()
          && let Ok(rel) = path.strip_prefix(&root)
        {
          let rel = rel.to_string_lossy();
          let rel = ifs::unix_path(rel);
          rel_len_ts.insert(
            rel,
            LenTs {
              ts: ts.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
              len: meta.len(),
            },
          );
        }
      }
    }

    Ok(Self { rel_len_ts, root })
  }
}
