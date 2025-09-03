use std::path::PathBuf;

#[static_init::dynamic]
pub static CACHE: PathBuf = std::env::var("XDG_CACHE_HOME")
  .map(|i| i.into())
  .unwrap_or_else(|_| {
    dirs::home_dir()
      .map(|i| i.join(".cache"))
      .unwrap_or_else(|| std::env::current_exe().unwrap().parent().unwrap().into())
  });
