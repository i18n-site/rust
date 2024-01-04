use aok::{Result, OK};
use enum_dispatch::enum_dispatch;

use crate::{
  db::{Kind, Watch},
  watch::Task,
};

macro_rules! hook {
  ($($type:ident),*) => {
    mod hook{
      $(mod $type;)*
    }

    #[allow(non_camel_case_types)]
    #[enum_dispatch(Task)]
    enum Hook {
      $($type,)*
    }

    $(
    #[allow(non_camel_case_types)]
    pub struct $type;

    impl Task for $type {
      async fn ping(&self) -> Result<()> {
        OK
      }
    }
    )*
  };
}

hook!(smtp, mysql);

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
  }
  hook!(smtp, mysql)
}
