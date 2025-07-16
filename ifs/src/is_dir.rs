use std::{fs, path::Path};

pub fn is_dir<P: AsRef<Path>>(path: P) -> Option<bool> {
  let path = path.as_ref();

  // 检查路径是否存在
  if path.exists() {
    // 检查路径是否是符号链接
    if path.is_symlink() {
      // 获取符号链接指向的原始路径
      match fs::read_link(path) {
        Ok(target) => {
          // 检查原始路径是否是目录
          Some(target.is_dir())
        }
        Err(_) => None,
      }
    } else {
      // 检查路径是否是目录
      Some(path.is_dir())
    }
  } else {
    None
  }
}
