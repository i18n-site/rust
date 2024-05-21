use std::mem::ManuallyDrop;

use aok::Result;
use redb::{Table, TableDefinition, TableError, WriteTransaction};

pub struct Txn {
  _w: ManuallyDrop<WriteTransaction>,
}

impl Drop for Txn {
  fn drop(&mut self) {
    let w = unsafe { ManuallyDrop::take(&mut self._w) };
    w.commit().unwrap();
  }
}

pub type MtimeLen<'a> = Table<'a, u16, (u64, u64)>;

impl Txn {
  pub fn table<'a>(&'a self, rel: &str, ext: &str) -> Result<MtimeLen<'a>, TableError> {
    let rel = &rel[..rel.len() - ext.len() - 1];

    let ml = format!("ml:{rel}");
    let ml = TableDefinition::new(&ml);

    self._w.open_table(ml)
  }

  pub fn open(db: &redb::Database) -> Result<Self> {
    let _w = db.begin_write()?;
    let me = Self {
      _w: ManuallyDrop::new(_w),
    };
    Ok(me)
  }
}
