#![feature(min_specialization)]
#![feature(macro_metavar_expr)]
#![feature(iter_intersperse)]
#![feature(trait_alias)]

mod mysql_val;
use std::fmt::Write;

use genv::def;
pub use mysql_async::{
  self, Conn, Error, FromRowError, FromValueError, Params, Result, ServerError,
  prelude::{FromRow, FromValue, Query, Queryable, WithParams},
};
use mysql_async::{OptsBuilder, Pool, SslOpts, prelude::StatementLike};
pub use mysql_val::MysqlVal;
pub use trt::bg;

#[derive(thiserror::Error, Debug)]
pub enum QueryError {
  #[error("q1 but none")]
  Q1Empty,
}

pub const MYSQL_DEFAULT_PORT: u16 = 3306;

def!(MYSQL_HOST: String | "127.0.0.1".to_owned());
def!(MYSQL_PORT:u16 | MYSQL_DEFAULT_PORT);
def!(MYSQL_USER, MYSQL_PWD, MYSQL_DB);
def!(MYSQL_COMPRESS:u8 | 0);
def!(MYSQL_SSL:String | "".to_owned());
def!(MYSQL_PREFER_SOCKET:bool | false);

#[static_init::dynamic]
pub static POOL: Pool = Pool::new({
  let mut build = OptsBuilder::default()
    .prefer_socket(MYSQL_PREFER_SOCKET())
    .ip_or_hostname(MYSQL_HOST())
    .tcp_port(MYSQL_PORT())
    .conn_ttl(std::time::Duration::from_secs(120))
    .user(Some(MYSQL_USER::<String>()))
    .pass(Some(MYSQL_PWD::<String>()))
    .db_name(Some(MYSQL_DB::<String>()));

  if !MYSQL_SSL().is_empty() {
    build = build.ssl_opts(SslOpts::default().with_danger_accept_invalid_certs(true));
  }
  let compress = MYSQL_COMPRESS();
  if compress > 0 {
    build = build.compression(mysql_async::Compression::new(compress as _));
  }
  build
});

#[macro_export]
macro_rules! arg {
  ($($arg:expr),+) => {
    vec![$($arg.into()),+]
  };
}

#[macro_export]
macro_rules! conn {
    ()=>{
        $crate::POOL.get_conn().await?
    };
    ($conn:expr;$func:ident $sql:expr, $($arg:expr),+) => {{
        use $crate::{Query, WithParams};
        $sql.with(
            $crate::Params::Positional(vec![$($arg.into()),+])
        ).$func($conn).await?
    }};
    ($conn:expr; $func:ident $sql:expr) => {{
        use $crate::{Query};
        $sql.$func($conn).await?
    }};
    ($func:ident $sql:expr $(,$arg:expr)* $(,)?) => {{
#[allow(unused_mut)]
let mut conn = $crate::conn!();
$crate::conn!(&mut conn; $func $sql $(,$arg)*)
    }};
}

macro_rules! def {
  ($($m:ident $func:ident;)+) => {
    $(def!($m $func);)+
  };
  ($m:ident $func:ident) => {
    #[macro_export]
    macro_rules! $m {
      ($conn:expr; $sql:expr $$(,$arg:expr)* $$(,)?) => {
        $$crate::conn!($$conn; $func $$sql $$(,$$arg)*)
      };
      ($sql:expr $$(,$arg:expr)* $$(,)?) => {
        $$crate::conn!($func $$sql $$(,$$arg)*)
      };
    }
  };
}

def!(
  e ignore;
  q fetch;
  q01 first;
);

pub fn args<T: Into<mysql_async::Value>>(
  li: impl IntoIterator<Item = T>,
) -> (Vec<mysql_async::Value>, String) {
  let mut r = String::new();
  let li = li
    .into_iter()
    .map(|i| {
      r.push_str("?,");
      i.into()
    })
    .collect::<Vec<_>>();
  if !r.is_empty() {
    r.truncate(r.len() - 1);
  }
  (li, r)
}

pub async fn q<R: FromRow + Send + 'static>(
  sql: impl StatementLike + 'static,
  arg: impl Into<MysqlVal>,
) -> Result<Vec<R>> {
  sql
    .with(Params::Positional(arg.into().0))
    .fetch::<R, _>(conn!())
    .await
}

pub async fn e(sql: impl StatementLike + 'static, arg: impl Into<MysqlVal>) -> Result<()> {
  sql
    .with(Params::Positional(arg.into().0))
    .ignore(conn!())
    .await
}

pub async fn last_id(sql: impl StatementLike, arg: impl Into<MysqlVal>) -> Result<u64> {
  Ok(last_id_or_none(sql, arg).await?.unwrap())
}

pub async fn last_id_or_none(
  sql: impl StatementLike,
  arg: impl Into<MysqlVal>,
) -> Result<Option<u64>> {
  let mut conn = conn!();
  let q = conn.exec_iter(sql, arg.into().0).await?;
  Ok(q.last_insert_id())
}

pub async fn q01<R: FromRow + Send + 'static>(
  sql: impl StatementLike + 'static,
  arg: impl Into<MysqlVal>,
) -> Result<Option<R>> {
  sql
    .with(Params::Positional(arg.into().0))
    .first(conn!())
    .await
}

#[macro_export]
macro_rules! bg {
    ($sql:expr $(,$arg:expr)* $(,)?) => {{
        $crate::bg(async move {
          $crate::e!($sql $(,$arg)*);
          Ok::<_,$crate::Error>(())
        });
    }};
}

#[macro_export]
macro_rules! q1 {
    ($conn:expr; $sql:expr $(,$arg:expr)* $(,)?) => {{
      if let Some(r) = $crate::q01!($conn;$sql $(,$arg)*) {
        r
      }else{
        Err($crate::Error::Other($crate::QueryError::Q1Empty.into()))?
      }
    }};
    ($sql:expr $(,$arg:expr)* $(,)?) => {{
        let mut conn = $crate::conn!();
        $crate::q1!(conn; $sql $(,$arg)*)
    }};
}

/*

因为这个错误
macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths

下面代码必须放到 lib.rs
*/

use gxhash::HashMap;
use xstr::Join;

pub trait IdType = ToString + std::cmp::Eq + std::hash::Hash + Copy + Send + 'static + FromValue;

pub async fn id_v<S: Send + 'static, Id: IdType>(
  table: &str,
  id_set: impl IntoIterator<Item = Id>,
) -> Result<HashMap<Id, S>>
where
  (Id, S): FromRow,
{
  let id_set = id_set.join(",");
  if id_set.is_empty() {
    return Ok(Default::default());
  }

  let li: Vec<(Id, S)> = q!(format!("SELECT id,v FROM {table} WHERE id IN ({})", id_set));
  Ok(HashMap::from_iter(li.into_iter()))
}

pub async fn id_v_str<Id: IdType>(
  table: &str,
  id_set: impl IntoIterator<Item = Id>,
) -> Result<HashMap<Id, String>>
where
  mysql_async::Value: From<<Id as mysql_async::prelude::FromValue>::Intermediate>,
{
  id_v(table, id_set).await
}

pub trait Id<S> {
  fn id(&self) -> S;
  fn col() -> &'static str;
}

pub async fn id_row<R: FromRow + Id<I> + Send + 'static, I: IdType>(
  table: &str,
  id_set: impl IntoIterator<Item = I>,
) -> Result<HashMap<I, R>> {
  let id_set = id_set.join(",");
  if id_set.is_empty() {
    return Ok(Default::default());
  }
  let row = R::col();
  let li: Vec<R> = q!(format!(
    "SELECT {row} FROM {table} WHERE id IN ({})",
    id_set
  ));
  Ok(HashMap::from_iter(li.into_iter().map(|i| (i.id(), i))))
}

pub fn s(s: impl AsRef<str>) -> String {
  let mut f = String::with_capacity(s.as_ref().len() + 2);
  f.write_str("'").unwrap();
  let s = s.as_ref();
  for part in s.split('\'').intersperse("''") {
    for part in part.split('\\').intersperse("\\\\") {
      f.write_str(part).unwrap();
    }
  }
  f.write_str("'").unwrap();
  f
}

pub fn b(bin: &[u8]) -> String {
  let mut s = String::with_capacity(bin.len() * 2 + 2);
  s.push_str("0x");
  for i in rustc_hex::ToHexIter::new(bin.iter()) {
    s.push(i);
  }
  s
}
