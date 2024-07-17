use std::{
  fs,
  path::{Path, PathBuf},
};

use crate::i18n_conf_path;

fn exists(path: &Path) -> bool {
  i18n_conf_path(path).exists()
}

// 辅助函数：递归检查子目录直到一定深度
fn subdir(base_dir: &Path, depth: usize, li: &mut Vec<PathBuf>) {
  // 如果深度为0，则返回
  if depth == 0 {
    return;
  }
  // 读取目录中的所有条目
  if let Ok(entries) = fs::read_dir(base_dir) {
    for entry in entries.flatten() {
      let path = entry.path();
      // 如果是目录，则递归检查
      if path.is_dir() {
        if exists(&path) {
          li.push(path.clone());
        }
        // 递归调用自身，深度减1
        subdir(&path, depth - 1, li);
      }
    }
  }
}

/// 函数：查找包含 `.i18n/conf.yml` 文件的目录
pub fn find_i18n_dir(dir: &Path) -> Vec<PathBuf> {
  let mut li = Vec::new();

  if exists(dir) {
    li.push(dir.to_path_buf());
  }

  let mut p = dir;
  // 检查所有上级目录
  while let Some(parent) = p.parent() {
    if exists(parent) {
      li.push(parent.to_path_buf());
    }
    p = parent;
  }

  // 检查当前目录的3层子目录
  subdir(dir, 3, &mut li);

  li
}

pub fn find_i18n_dir_or_exit(root: &Path) -> Vec<PathBuf> {
  let li = find_i18n_dir(root);
  if li.is_empty() {
    eprintln!(".conf/i18n.yml NOT EXIST");
    std::process::exit(1);
  }
  li
}
