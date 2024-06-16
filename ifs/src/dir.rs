use std::path::PathBuf;

#[static_init::dynamic]
pub static BIN_HOME: PathBuf = std::env::var("XDG_BIN_HOME")
  .map(|i| i.into())
  .unwrap_or_else(|_| {
    dirs::home_dir()
      .map(|i| i.join(".local/bin"))
      .unwrap_or_else(|| std::env::current_exe().unwrap().parent().unwrap().into())
  });
