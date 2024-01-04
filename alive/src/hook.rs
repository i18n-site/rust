use aok::{Result, OK};

pub async fn smtp() -> Result<()> {
  OK
}

macro_rules! hook {
  ($($fn:ident),*) => {
    pub fn hook(name: &str) -> Option<impl futures::Future<Output=Result<()>>> {
      match name {
        $(
          stringify!($fn) => {
            Some(crate::watch($fn()))
          }
        ),*
        _ => None,
      }
    }
  };
}

hook!(smtp);
