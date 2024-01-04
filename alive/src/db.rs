use mysql_macro::{exe, mysql_async::prelude::FromRow, q, q01};

#[derive(Debug, Clone, FromRow)]
pub struct Kind {
  pub id: u64,
  pub url_id: u64,
  pub duration: u32,
  pub warnErr: u8,
  pub v: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct Watch {
  pub id: u64,
  pub host_id: u64,
  pub kind_id: u64,
  pub dns_type: u8,
  pub err: u32,
  pub url_id: u64,
}
