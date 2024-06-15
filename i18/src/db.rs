use std::{ops::Deref, path::Path};

use fjall::{Config, Keyspace, PartitionHandle};

pub struct Table<'a> {
  pub db: &'a Keyspace,
  pub table: PartitionHandle,
}

pub struct Db(pub Keyspace);

pub fn open(fp: &Path) -> Result<Db, fjall::Error> {
  let fjall = Config::new(fp).open()?;
  Ok(Db(fjall))
}

impl Db {
  pub fn table<'a>(&'a self, table: &'a str) -> Result<Table<'a>, fjall::Error> {
    Ok(Table {
      db: &self.0,
      table: self.0.open_partition(table, Default::default())?,
    })
  }

  // pub fn flush(&self) -> Result<(), fjall::Error> {
  //   self.0.persist(FlushMode::SyncAll)?;
  //   Ok(())
  // }
}

impl Deref for Table<'_> {
  type Target = PartitionHandle;
  fn deref(&self) -> &Self::Target {
    &self.table
  }
}
