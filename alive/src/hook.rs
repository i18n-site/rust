use aok::{Result, OK};
use enum_dispatch::enum_dispatch;

use crate::{
  db::{Kind, Watch},
  watch::Task,
};

#[allow(non_camel_case_types)]
#[enum_dispatch(Task)]
enum Hook {
  smtp,
  mysql,
}

#[allow(non_camel_case_types)]
pub struct smtp;

impl Task for smtp {
  async fn run(&self) -> Result<()> {
    OK
  }
}

#[allow(non_camel_case_types)]
pub struct mysql;

impl Task for mysql {
  async fn run(&self) -> Result<()> {
    OK
  }
}

pub fn hook<'a>(
  kind: &'a Kind,
  watch: &'a Watch,
  host: &'a str,
) -> Option<impl futures::Future<Output = Result<()>> + 'a> {
  macro_rules! hook {
  ($($fn:ident),*) => {
      match kind.v.as_str() {
        $(
          stringify!($fn) => {
            Some(crate::watch(kind,watch,host,Hook::$fn($fn)))
          }
        ),*
        _ => None,
      }
    }
  };
  hook!(smtp, mysql)
}
