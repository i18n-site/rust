use std::{ops::Deref, path::Path};

use fjall::{Database, Keyspace, KeyspaceCreateOptions};

pub struct Table<'a> {
  pub db: &'a Database,
  pub table: Keyspace,
}

pub struct Db(pub Database);

pub fn open(fp: impl AsRef<Path>) -> Result<Db, fjall::Error> {
  let fjall = Database::builder(fp.as_ref()).open()?;
  Ok(Db(fjall))
}

impl Db {
  pub fn table<'a>(&'a self, table: &'a str) -> Result<Table<'a>, fjall::Error> {
    Ok(Table {
      db: &self.0,
      table: self.0.keyspace(table, KeyspaceCreateOptions::default())?,
    })
  }

  // pub fn flush(&self) -> Result<(), fjall::Error> {
  //   self.0.persist(PersistMode::SyncAll)?;
  //   Ok(())
  // }
}

impl Deref for Table<'_> {
  type Target = Keyspace;
  fn deref(&self) -> &Self::Target {
    &self.table
  }
}
