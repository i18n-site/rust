use aok::{Result, OK};

pub async fn smtp() -> Result<()> {
  OK
}

async fn log(result: Result<()>) {
  xerr::log!(async move { result }.await);
}

macro_rules! hook {
  ($($fn:ident),*) => {
    pub async fn hook(name: &str) -> bool {
      match name {
        $(
          stringify!($fn) => {
            log(crate::watch($fn().await).await);
            true
          }
        ),*
        _ => false,
      }
    }
  };
}

hook!(smtp);
