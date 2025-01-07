use std::{fs::remove_file, os, path::Path};

pub fn file<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> std::io::Result<()> {
  let link = link.as_ref();
  if link.exists() {
    remove_file(link)?;
  }
  #[cfg(target_os = "windows")]
  os::windows::fs::symlink_file(original, link)?;

  #[cfg(not(target_os = "windows"))]
  os::unix::fs::symlink(original, link)?;

  Ok(())
}
