#![feature(macro_metavar_expr)]

use genv::def;
pub use mysql_async::{
  self,
  prelude::{Query, Queryable, WithParams},
  Error, Params, Result,
};
use mysql_async::{OptsBuilder, Pool};
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
    exe ignore;
    q fetch;
    q01 first;
);

#[macro_export]
macro_rules! bg {
    ($sql:expr $(,$arg:expr)* $(,)?) => {{
        $crate::bg(async move {
          $crate::exe!($sql $(,$arg)*);
          Ok::<_,$crate::Error>(())
        });
    }};
}

#[macro_export]
macro_rules! q1 {
    ($conn:expr; $sql:expr $(,$arg:expr)* $(,)?) => {
        $crate::q01!($conn;$sql $(,$arg)*).unwrap()
    };
    ($sql:expr $(,$arg:expr)* $(,)?) => {
        $crate::q01!($sql $(,$arg)*).unwrap()
    };
}

/*

因为这个错误
macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths

下面代码必须放到 lib.rs
*/

use std::collections::HashMap;

use xstr::Join;

pub async fn id_v<S: Send + 'static>(
  table: &str,
  id_set: impl IntoIterator<Item = u64>,
) -> Result<HashMap<u64, S>>
where
  (u64, S): mysql_async::prelude::FromRow,
{
  let id_set = id_set.join(",");
  if id_set.is_empty() {
    return Ok(Default::default());
  }

  let li: Vec<(u64, S)> = q!(format!("SELECT id,v FROM {table} WHERE id IN ({})", id_set));
  Ok(HashMap::from_iter(li.into_iter()))
}

pub async fn id_v_str(
  table: &str,
  id_set: impl IntoIterator<Item = u64>,
) -> Result<HashMap<u64, String>> {
  id_v(table, id_set).await
}
