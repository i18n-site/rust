use aok::Result;

use crate::db::{Kind, Watch};

macro_rules! hook {
  ($($type:ident),*) => {
$(mod $type;)*

mod hook {
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
      async fn ping(&self) -> Result<()> {
        super::$type::ping().await
      }
    }
  )*
}
pub fn hook<'a>(
  kind: &'a Kind,
  watch: &'a Watch,
  host: &'a str,
  kind_url: &'a str,
  watch_url: &'a str,
) -> Option<impl futures::Future<Output = Result<()>> + 'a> {
  match kind.v.as_str() {
    $(
      stringify!($fn) => {
        Some(crate::watch(kind,watch,host,self::hook::Hook::$type(self::hook::$type)))
      }
    ),*
    _ => None,
  }
}
  };
}

hook!(smtp, mysql);
