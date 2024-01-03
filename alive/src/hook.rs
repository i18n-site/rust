pub async fn smtp() -> Result<()> {
  OK
}

macro_rules! hook {
  ($($fn:ident),*) => {
    pub async fn hook(name: &str) -> Result<bool> {
      match name {
        $(
          stringify!($fn) => {
            $fn().await?;
            Ok(true)
          }
        ),*
        _ => Ok(false),
      }
    }
  };
}

hook!(smtp);
