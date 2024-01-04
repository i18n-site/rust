use aok::Result;

use crate::db::{Kind, Watch};

macro_rules! hook {
  ($($type:ident),*) => {
$(mod $type;)*

mod hook {
  use crate::db::{Kind, Watch};
  use enum_dispatch::enum_dispatch;
  use aok::{Result};
  use crate::watch::Task;

  #[allow(non_camel_case_types)]
  #[enum_dispatch(Task)]
  pub enum Hook {
    $($type,)*
  }

  $(
    #[allow(non_camel_case_types)]
    pub struct $type;

    impl Task for $type {
      async fn ping<'a>(&self,
        kind: &'a Kind,
        watch: &'a Watch,
        host: &'a str,
        kind_arg: &'a str,
        watch_arg: &'a str,
      ) -> Result<()> {
        super::$type::ping(kind,watch,host,kind_arg,watch_arg).await
      }
    }
  )*
}
pub fn hook<'a>(
  kind: &'a Kind,
  watch: &'a Watch,
  host: &'a str,
  kind_arg: &'a str,
  watch_arg: &'a str,
) -> Option<impl futures::Future<Output = Result<()>> + 'a> {
  match kind.v.as_str() {
    $(
      stringify!($fn) => {
        Some(crate::watch(kind,watch,host,kind_arg,watch_arg,self::hook::Hook::$type(self::hook::$type)))
      }
    ),*
    _ => None,
  }
}
  };
}

hook!(smtp, mysql);
