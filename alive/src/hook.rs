use aok::{Result, OK};

pub async fn smtp() -> Result<()> {
  OK
}

macro_rules! hook {
  ($($fn:ident),*) => {
    pub async fn hook(name: &str) -> bool {
      match name {
        $(
          stringify!($fn) => {
            watch($fn().await).await;
            true
          }
        ),*
        _ => false,
      }
    }
  };
}

hook!(smtp);
