use std::{
  collections::HashMap,
  fs::{File, Metadata},
  hash::Hasher,
  io::{BufRead, BufReader, Read},
  path::{Path, PathBuf},
};

use aok::{OK, Void};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
  pub len_ts: LenTs,
  pub hash: u128,
}

pub fn path_meta(path: impl AsRef<Path>) -> std::io::Result<Meta> {
  let len_ts = LenTs::new(std::fs::metadata(path.as_ref())?);
  let hash = hash(path.as_ref())?;
  Ok(Meta { len_ts, hash })
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
  pub root: PathBuf,
}

fn meta_encode(rel: impl AsRef<str>, meta: &Meta) -> pc::Result<String> {
  Ok(format!(
    "{}#{}",
    rel.as_ref(),
    burl::e(pc::e::<Meta>(meta)?)
  ))
}

impl Diff {
  pub fn new(root: impl Into<PathBuf>, db: impl Into<PathBuf>) -> Self {
    Self {
      root: root.into(),
      db: db.into(),
      changed: vec![],
      no_change: vec![],
      has_change: false,
    }
  }

  pub fn refresh(&mut self, path: impl Into<String>) -> Void {
    let path = path.into();
    let meta = path_meta(self.root.join(&path))?;

    let p = self.no_change.iter().position(|(p, _)| *p == path);

    if let Some(p) = p {
      let m = &self.no_change[p].1;
      if m.len_ts.len == meta.len_ts.len && m.len_ts.ts == meta.len_ts.ts {
        return OK;
      }
      self.no_change.remove(p);
    }

    self.changed.retain(|(p, _)| *p != path);
    self.changed.push((path, meta));
    self.has_change = true;

    // let line = meta_encode(rel, &meta)?;

    // 判断self.db 是否存在,如果存在就追加"\n{line}"到文件末尾,否则写入line到文件
    // if self.db.exists() {
    //   let mut f = std::fs::OpenOptions::new().append(true).open(&self.db)?;
    //
    //   f.write_all(b"\n")?;
    //   f.write_all(line.as_bytes())?;
    // } else {
    //   ifs::wstr(&self.db, line)?;
    // }

    OK
  }

  pub fn save(&self) -> Void {
    if !self.has_change {
      return OK;
    }
    let mut li = vec![];
    for (rel, meta) in self.changed.iter().chain(self.no_change.iter()) {
      li.push(meta_encode(rel, meta)?);
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
      // 设计为可追加的模式, 后面的记录会覆盖到前面的
      let mut rel_meta = HashMap::new();

      for line in file.lines().map_while(Result::ok) {
        let line = line.trim_end();
        if let Some(i) = line.chars().next() {
          if "<>#".contains(i) {
            continue;
          }

          if let Some(pos) = line.rfind('#') {
            let bin = &line[pos + 1..];
            if let Ok(meta) = burl::d(bin)
              && let Ok::<Meta, _>(meta) = pc::d(&meta)
            {
              rel_meta.insert(line[..pos].to_owned(), meta);
            }
          }
        }
      }
      for (rel, meta) in rel_meta {
        if let Some(len_ts) = rel_len_ts.remove(&rel) {
          if len_ts.len == meta.len_ts.len && len_ts.ts == meta.len_ts.ts {
            no_change.push((rel, meta));
          } else {
            let fp = self.root.join(&rel);
            let hash = hash(&fp)?;
            if hash == meta.hash {
              set_mtime(&fp, meta.len_ts.ts)?;
              no_change.push((
                rel,
                Meta {
                  len_ts: LenTs {
                    len: len_ts.len,
                    ts: meta.len_ts.ts,
                  },
                  hash,
                },
              ));
            } else {
              changed.push((rel, Meta { len_ts, hash }));
            }
          }
        }
      }
    }

    // 新的文件
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
