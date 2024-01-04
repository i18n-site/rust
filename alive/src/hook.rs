use aok::{Result, OK};

use crate::db::{Kind, Watch};

pub async fn smtp() -> Result<()> {
  OK
}

pub async fn test() -> Result<()> {
  OK
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
            Some(crate::watch(kind,watch,host,$fn()))
          }
        ),*
        _ => None,
      }
    }
  };
  hook!(smtp, test)
}
