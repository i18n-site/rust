use std::path::PathBuf;

use current_platform::CURRENT_PLATFORM as TARGET;

pub fn project(name: &str) -> PathBuf {
  std::env::home_dir()
    .unwrap_or("/".into())
    .join(format!(".{}", name))
    .join(TARGET)
}
