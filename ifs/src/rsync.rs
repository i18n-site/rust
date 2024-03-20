use std::{fs, path::PathBuf};

use aok::{Result, OK};
use walkdir::WalkDir;

pub fn rsync(from: impl Into<PathBuf>, to_dir: impl Into<PathBuf>) -> Result<()> {
  let from = from.into();
  let to_dir = to_dir.into();
  for i in WalkDir::new(&from) {
    if let Ok(i) = xerr::ok!(i) {
      let file_type = i.file_type();
      let is_file = file_type.is_file();
      let is_dir = file_type.is_dir();
      if is_file || is_dir {
        let path = i.path();
        let rel = path.strip_prefix(&from)?;
        let to = to_dir.join(rel);
        if is_dir {
          crate::ensure_file_dir(&to)?;
        } else if is_file {
          fs::copy(i.path(), to)?;
        }
      }
    }
  }
  OK
}
