#![feature(macro_metavar_expr)]
#![feature(iter_intersperse)]
#![feature(trait_alias)]

use std::fmt::Write;

use genv::def;
pub use mysql_async::{
  self,
  prelude::{FromRow, FromValue, Query, Queryable, WithParams},
  Conn, Error, FromRowError, FromValueError, Params, Result, ServerError,
};
use mysql_async::{prelude::StatementLike, OptsBuilder, Pool};
pub use trt::bg;

pub const MYSQL_DEFAULT_PORT: u16 = 3306;

def!(MYSQL_HOST: String | "127.0.0.1".to_owned());
def!(MYSQL_PORT:u16 | MYSQL_DEFAULT_PORT);
def!(MYSQL_USER, MYSQL_PWD, MYSQL_DB);
def!(MYSQL_COMPRESS:u8 | 0);

#[static_init::dynamic]
pub static POOL: Pool = Pool::new({
  let mut build = OptsBuilder::default()
    .ip_or_hostname(MYSQL_HOST())
    .tcp_port(MYSQL_PORT())
    .user(Some(MYSQL_USER::<String>()))
    .pass(Some(MYSQL_PWD::<String>()))
    .db_name(Some(MYSQL_DB::<String>()));
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

pub async fn q<R: FromRow + Send + 'static>(
  sql: impl StatementLike + 'static,
  arg: Vec<mysql_async::Value>,
) -> Result<Vec<R>> {
  sql
    .with(Params::Positional(arg))
    .fetch::<R, _>(conn!())
    .await
}

pub async fn e(sql: impl StatementLike + 'static, arg: Vec<mysql_async::Value>) -> Result<()> {
  sql.with(Params::Positional(arg)).ignore(conn!()).await
}

pub async fn last_id(sql: impl StatementLike, arg: Vec<mysql_async::Value>) -> Result<u64> {
  Ok(last_id_or_none(sql, arg).await?.unwrap())
}

pub async fn last_id_or_none(
  sql: impl StatementLike,
  arg: Vec<mysql_async::Value>,
) -> Result<Option<u64>> {
  let mut conn = conn!();
  let q = conn.exec_iter(sql, arg).await?;
  Ok(q.last_insert_id())
}

pub async fn q01<R: FromRow + Send + 'static>(
  sql: impl StatementLike + 'static,
  arg: Vec<mysql_async::Value>,
) -> Result<Option<R>> {
  sql.with(Params::Positional(arg)).first(conn!()).await
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
    ($conn:expr; $sql:expr $(,$arg:expr)* $(,)?) => {
        $crate::q01!($conn;$sql $(,$arg)*).unwrap()
    };
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

use std::collections::HashMap;

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
