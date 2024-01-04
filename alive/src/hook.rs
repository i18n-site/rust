use aok::{Result, OK};

use crate::db::{Kind, Watch};

pub async fn smtp() -> Result<()> {
  OK
}

// pub async fn test() -> Result<()> {
//   OK
// }

macro_rules! hook {
  ($($fn:ident),*) => {
    pub fn hook<'a>(kind: &'a Kind, watch:&'a Watch) -> Option<impl futures::Future<Output=Result<()>>+'a> {
      match kind.v.as_str() {
        $(
          stringify!($fn) => {
            Some(crate::watch(kind,watch,$fn()))
          }
        ),*
        _ => None,
      }
    }
  };
}

hook!(smtp);
