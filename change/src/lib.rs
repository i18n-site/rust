#![feature(let_chains)]
#![feature(str_split_remainder)]

use std::{
  fs::File,
  hash::Hasher,
  io::{BufRead, BufReader, Read},
  path::{Path, PathBuf},
};

use aok::{Null, OK};
use bincode::{Decode, Encode};
use gxhash::HashMap;
use set_mtime::set_mtime;
pub use walkdir::WalkDir;
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

#[derive(Debug, Clone, Encode, Decode)]
pub struct LenTs {
  pub ts: u64,
  pub len: u64,
}

#[derive(Debug, Encode, Decode)]
pub struct Meta {
  pub len_ts: LenTs,
  pub hash: u128,
}

#[derive(Debug)]
pub struct Scan {
  pub public: PathBuf,
  pub rel_len_ts: HashMap<String, LenTs>,
}

pub struct State {
  pub changed: Vec<(String, Meta)>,
  pub no_change: Vec<(String, Meta)>,
  pub db: PathBuf,
  pub has_change: bool,
}

impl State {
  pub fn save(&self) -> Null {
    let mut li = vec![];
    for (rel, meta) in self.changed.iter().chain(&self.no_change) {
      let meta = burl::e(bce::e(meta)?);
      li.push(format!("{rel}#{meta}"));
    }

    let db = li.join("\n");
    li.sort();

    ifs::wtxt(&self.db, db)?;

    OK
  }
}

impl Scan {
  pub fn change(&self, db: impl Into<PathBuf>) -> std::io::Result<State> {
    let mut changed = vec![];
    let mut no_change = vec![];
    let db = db.into();
    let mut rel_len_ts = self.rel_len_ts.clone();
    if db.exists() {
      let file = std::io::BufReader::new(std::fs::File::open(&db)?);
      for line in file.lines().flatten() {
        let line = line.trim_end();
        if let Some(i) = line.chars().next() {
          if "<>#".contains(i) {
            continue;
          } else if let Some(pos) = line.rfind('#') {
            let bin = &line[pos + 1..];
            if let Ok(meta) = burl::d(bin) {
              if let Ok::<Meta, _>(meta) = bce::d(&meta) {
                let rel = &line[..pos];
                if let Some(len_ts) = rel_len_ts.remove(rel) {
                  if len_ts.len == meta.len_ts.len && len_ts.ts == meta.len_ts.ts {
                    no_change.push((rel.into(), meta));
                  } else {
                    let fp = self.public.join(rel);
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
                      changed.push((rel.into(), meta));
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
      let fp = self.public.join(&rel);
      changed.push((
        rel,
        Meta {
          len_ts,
          hash: hash(fp)?,
        },
      ));
    }

    Ok(State {
      has_change: changed.is_empty() && no_change.len() == self.rel_len_ts.len(),
      changed,
      db,
      no_change,
    })
  }

  pub fn new(public: impl Into<PathBuf>) -> std::io::Result<Self> {
    let public = public.into();
    let mut rel_len_ts = HashMap::default();

    for entry in WalkDir::new(&public).into_iter() {
      if let Ok(entry) = entry
        && entry.file_type().is_file()
      {
        let path = entry.path();
        if let Ok(meta) = std::fs::metadata(path)
          && let Ok(ts) = meta.modified()
          && let Ok(rel) = path.strip_prefix(&public)
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

    Ok(Self { rel_len_ts, public })
  }
}
