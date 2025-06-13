use std::path::PathBuf;

pub fn confdir() -> PathBuf {
  std::env::var("XDG_CONFIG_HOME")
    .map(|i| i.into())
    .unwrap_or_else(|_| dirs::home_dir().unwrap().join(".config"))
}
