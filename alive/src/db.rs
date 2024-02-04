use m::{mysql_async::prelude::FromRow, Id};

#[derive(Debug, Clone, FromRow)]
pub struct Kind {
  pub id: u64,
  pub arg_id: u64,
  pub duration: u32,
  pub warnErr: u8,
  pub v: String,
}

impl Id<u64> for Kind {
  fn col() -> &'static str {
    "id,arg_id,duration,warnErr,v"
  }

  fn id(&self) -> u64 {
    self.id
  }
}

#[derive(Debug, Clone, FromRow)]
pub struct Watch {
  pub id: u64,
  pub host_id: u64,
  pub kind_id: u64,
  pub dns_type: u8,
  pub err: u32,
  pub arg_id: u64,
}

#[derive(Debug, Clone, FromRow)]
pub struct Status {
  pub host_id: u64,
  pub kind_id: u64,
  pub dns_type: u8,
  pub err: u32,
  pub ts: u64,
}
