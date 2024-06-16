use std::{
  collections::{HashMap, HashSet},
  fs,
  path::{Path, PathBuf},
};

pub fn is(m: &clap::ArgMatches) -> bool {
  *m.get_one("purge").unwrap_or(&false)
}

#[macro_export]
macro_rules! purge_arg {
  ()=>{
    arg!(-p --purge "purge file in to-lang dir where it not exist in from-lang dir")
  }
}
// pub fn cli() -> Result<bool> {
//   let purge: bool = m.get_one("purge").unwrap_or(&false).clone();
//   if purge {}
//   Ok(purge)
// }

pub fn purge(root: &Path, from_to: &HashMap<String, String>) -> std::io::Result<()> {
  let ft: ft::FromTo = from_to.into();
  if let Some(src) = ft.root() {
    let to_li: Vec<_> = ft
      .all_lang_li()
      .into_iter()
      .filter(|s| *s != src)
      .map(|i| root.join(lang::LANG_CODE[i as usize]))
      .collect();
    let src = root.join(lang::LANG_CODE[src as usize]);
    purge_base_dir_li(&src, to_li)?;
  }
  Ok(())
}

pub fn purge_base_dir_li(base: &Path, purge_dir_li: Vec<PathBuf>) -> std::io::Result<()> {
  // 创建一个 HashSet 存储 base 目录下的所有文件和子目录
  let mut exists = HashSet::new();

  // 遍历 base 目录及其子目录
  collect(base, base, &mut exists)?;

  for purge_dir in purge_dir_li {
    for entry in fs::read_dir(&purge_dir)?.flatten() {
      let path = entry.path();
      rm(&path, &purge_dir, &exists)?;
    }
  }

  Ok(())
}

// 递归收集 base 目录下的所有文件和子目录
fn collect(path: &Path, root_dir: &Path, exists: &mut HashSet<PathBuf>) -> std::io::Result<()> {
  if path.is_dir() {
    for entry in fs::read_dir(path)? {
      let entry = entry?;
      let path = entry.path();
      if let Ok(p) = path.strip_prefix(root_dir) {
        exists.insert(p.to_path_buf());
      }
      collect(&path, root_dir, exists)?;
    }
  } else if path.is_file() {
    if let Ok(rel) = path.strip_prefix(root_dir) {
      exists.insert(rel.to_path_buf());
    }
  }
  Ok(())
}

// 递归清理 purge_dir 目录下不存在于 base 目录中的文件和子目录
fn rm(path: &Path, root_dir: &Path, exists: &HashSet<PathBuf>) -> std::io::Result<()> {
  if path.is_dir() {
    let rel = path.strip_prefix(root_dir).ok().map(|p| p.to_path_buf());
    if let Some(ref rel_path) = rel {
      if !exists.contains(rel_path) {
        fs::remove_dir_all(path)?;
      } else {
        for entry in fs::read_dir(path)? {
          let entry = entry?;
          let path = entry.path();
          rm(&path, root_dir, exists)?;
        }
      }
    }
  } else if path.is_file() {
    if let Ok(rel) = path.strip_prefix(root_dir) {
      if !exists.contains(&rel.to_path_buf()) {
        fs::remove_file(path)?;
      }
    }
  }
  Ok(())
}
