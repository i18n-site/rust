use std::{fs, path::PathBuf};

use aok::{OK, Result};
use walkdir::DirEntry;
pub use walkdir::WalkDir;

/// 将一个目录的内容同步到另一个目录
pub fn rsync(
  from: impl Into<PathBuf>,
  walk: impl IntoIterator<Item = walkdir::Result<DirEntry>>,
  to_dir: impl Into<PathBuf>,
) -> Result<()> {
  let from = from.into();
  let to_dir = to_dir.into();
  if !to_dir.exists() {
    fs::create_dir_all(&to_dir)?;
  }
  for i in walk {
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
