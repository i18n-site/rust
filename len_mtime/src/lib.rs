use std::path::{Path, PathBuf};

use aok::Result;
use bytes::Bytes;
use gxhash::HashMap;
use speedy::{Readable, Writable};
use trait_len::Len;

use crate::db::{Db, Table};
pub mod db;

#[derive(Readable, Writable, Debug, Eq, PartialEq)]
pub struct File {
  pub len: u64,
  pub mtime: u64,
  pub meta: Box<[u8]>,
}

pub struct LenMtime<'a> {
  pub root: PathBuf,
  pub db: Box<Db>,
  pub table: Table<'a>,
  pub changed: HashMap<String, File>,
}

impl<'a> LenMtime<'a> {
  pub fn load(db: impl AsRef<Path>, root: impl Into<PathBuf>) -> Result<Self, fjall::Error> {
    let db = Box::new(db::open(db.as_ref())?);
    let table = db.table("lm")?;
    let table: Table<'a> = unsafe { std::mem::transmute(table) };
    Ok(Self {
      table,
      db,
      root: root.into(),
      changed: Default::default(),
    })
  }

  pub fn is_change<S: AsRef<str>, B: Clone + Into<Bytes>>(
    &mut self,
    li: impl IntoIterator<Item = (S, B)>,
  ) -> Result<Vec<(String, B)>> {
    let table = &self.table;
    let mut changed = vec![];
    for (rel, m) in li {
      let meta: Box<[u8]> = m.clone().into().as_ref().into();
      let rel = rel.as_ref();
      if let Some((len, mtime)) = ifs::len_mtime(self.root.join(rel)) {
        let file = File { len, mtime, meta };
        if let Some(pre) = table.get(rel)? {
          let file_bin = file.write_to_vec()?;
          if *file_bin == *pre {
            continue;
          }
        }
        let rel: String = rel.into();
        changed.push((rel.clone(), m));
        self.changed.insert(rel, file);
      }
    }
    Ok(changed)
  }

  pub fn write<T: Len + IntoIterator<Item = (String, File)>>(
    &mut self,
    rel_bin: T,
  ) -> Result<(), fjall::Error> {
    if !rel_bin.is_empty() {
      let mut batch = self.db.0.batch();
      for (rel, file) in rel_bin {
        if let Ok(bin) = xerr::ok!(file.write_to_vec()) {
          batch.insert(&self.table, rel, bin);
        }
      }
      return batch.commit();
    }
    Ok(())
  }

  pub fn save<S: AsRef<str>>(
    &mut self,
    path_li: impl IntoIterator<Item = S>,
  ) -> Result<(), fjall::Error> {
    let mut li = vec![];
    for rel in path_li {
      let rel = rel.as_ref();
      if let Some(file) = self.changed.remove(rel) {
        li.push((rel.to_owned(), file));
      }
    }
    self.write(li)?;
    Ok(())
  }

  pub fn save_fp<S: AsRef<str>>(
    &mut self,
    path_li: impl IntoIterator<Item = S>,
  ) -> Result<(), fjall::Error> {
    let mut li = vec![];
    for rel in path_li {
      let rel = rel.as_ref();
      if let Some(mut file) = self.changed.remove(rel) {
        if let Some((len, mtime)) = ifs::len_mtime(self.root.join(rel)) {
          file.len = len;
          file.mtime = mtime;
          li.push((rel.to_owned(), file));
        }
      }
    }
    self.write(li)?;
    Ok(())
  }
}
