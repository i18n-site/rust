use std::{fs, io, path::Path};

use globset::GlobSet;
use gxhash::HashSet;
use i18_conf::I18nConf;
use ifs::is_dir;
use lang::LANG_CODE;
use walkdir::WalkDir;

use crate::Scan;

pub fn purge_dir(root: &Path, rel_set: &HashSet<String>) -> io::Result<()> {
  let mut directories = vec![];
  let mut to_delete = vec![];

  // Traverse the directory tree
  for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
    let path = entry.path();
    if path.is_file() {
      // Create relative path from root
      let rel_path = path
        .strip_prefix(root)
        .unwrap()
        .to_string_lossy()
        .to_string();
      // If file is not in rel_set, mark it for deletion
      if !rel_set.contains(&rel_path) {
        to_delete.push(path.to_path_buf());
      }
    } else if path.is_dir() {
      directories.push(path.to_path_buf());
    }
  }

  // Delete files
  for file in to_delete {
    fs::remove_file(file)?;
  }

  // Reverse the directories list to delete empty directories from leaf to root
  directories.reverse();
  for dir in directories {
    if fs::read_dir(&dir)?.next().is_none() {
      fs::remove_dir(dir)?;
    }
  }

  Ok(())
}

pub fn is(m: &clap::ArgMatches) -> bool {
  *m.get_one("purge").unwrap_or(&false)
}

#[macro_export]
macro_rules! purge_arg {
  ()=>{
    arg!(-p --purge "purge file in to-lang dir where it not exist in from-lang dir")
  }
}

fn subdir(root: &Path) -> io::Result<Vec<String>> {
  let mut li = Vec::new();

  for entry in fs::read_dir(root)? {
    let entry = entry?;
    let path = entry.path();
    if path.is_dir()
      && let Ok(path) = path.strip_prefix(root)
    {
      li.push(path.to_string_lossy().into());
    }
  }

  Ok(li)
}

// pub fn cli() -> Result<bool> {
//   let purge: bool = m.get_one("purge").unwrap_or(&false).clone();
//   if purge {}
//   Ok(purge)
// }

pub fn purge(root: &Path, conf: &I18nConf) -> std::io::Result<()> {
  // let mut prefix_li: Vec<_> = conf.path.iter().map(|i| i.0).collect();
  // prefix_li.sort_by_key(|s| Reverse(s.len()));
  //
  // _purge(root, "", &prefix_li, &conf.fromTo)?;
  // for prefix in prefix_li {}
  // unix_path
  // dbg!(&d);

  if let Some(is) = is_dir(root)
    && is
  {
    let empty_ignore = GlobSet::default();
    let rel_set = Scan::new(root, conf, &empty_ignore).rel_set();
    let ft = ft::FromTo::from_iter(conf.fromTo.iter());
    let all_lang_set: HashSet<_> = ft
      .all_lang_li()
      .into_iter()
      .map(|i| LANG_CODE[i as usize])
      .collect();
    for i in subdir(root)?
      .into_iter()
      .filter(|s| LANG_CODE.contains(&s.as_str()))
    {
      if all_lang_set.contains(i.as_str()) {
        purge_dir(&root.join(i), &rel_set)?;
      } else {
        let path = root.join(i);
        fs::remove_dir_all(&path)?;
      }
    }
  }

  Ok(())
}

// pub fn _purge(
//   root: &Path,
//   prefix: &str,
//   prefix_li: &[&String],
//   from_to: &HashMap<String, String>,
// ) -> std::io::Result<()> {
//   let ft: ft::FromTo = from_to.into();
//   if let Some(src) = ft.root() {
//     let to_li: Vec<_> = ft
//       .all_lang_li()
//       .into_iter()
//       .filter(|s| *s != src)
//       .map(|i| root.join(lang::LANG_CODE[i as usize]))
//       .collect();
//     let src = root.join(lang::LANG_CODE[src as usize]);
//     if let Some(is_dir) = is_dir(&src) {
//       // 如果是单个文件, 就不用清理 (不管是否存在都不用删除)
//       if is_dir {
//         purge_base_dir_li(root, prefix, prefix_li, &src, to_li)?;
//       }
//     }
//   }
//   Ok(())
// }

// pub fn purge_base_dir_li(
//   root: &Path,
//   prefix: &str,
//   prefix_li: &[&String],
//   base: &Path,
//   purge_dir_li: Vec<PathBuf>,
// ) -> std::io::Result<()> {
//   // 创建一个 HashSet 存储 base 目录下的所有文件和子目录
//   let mut exists = HashSet::new();
//
//   // 遍历 base 目录及其子目录
//   collect(base, base, &mut exists)?;
//
//   for purge_dir in purge_dir_li {
//     for entry in fs::read_dir(&purge_dir)?.flatten() {
//       let path = entry.path();
//       rm(&path, &purge_dir, &exists)?;
//     }
//   }
//
//   Ok(())
// }

// // 递归收集 base 目录下的所有文件和子目录
// fn collect(path: &Path, root_dir: &Path, exists: &mut HashSet<PathBuf>) -> std::io::Result<()> {
//   if path.is_dir() {
//     for entry in fs::read_dir(path)? {
//       let entry = entry?;
//       let path = entry.path();
//       if let Ok(p) = path.strip_prefix(root_dir) {
//         exists.insert(p.to_path_buf());
//       }
//       collect(&path, root_dir, exists)?;
//     }
//   } else if path.is_file() {
//     if let Ok(rel) = path.strip_prefix(root_dir) {
//       exists.insert(rel.to_path_buf());
//     }
//   }
//   Ok(())
// }

// // 递归清理 purge_dir 目录下不存在于 base 目录中的文件和子目录
// fn rm(path: &Path, root_dir: &Path, exists: &HashSet<PathBuf>) -> std::io::Result<()> {
//   if path.is_dir() {
//     let rel = path.strip_prefix(root_dir).ok().map(|p| p.to_path_buf());
//     if let Some(ref rel_path) = rel {
//       if !exists.contains(rel_path) {
//         fs::remove_dir_all(path)?;
//       } else {
//         for entry in fs::read_dir(path)? {
//           let entry = entry?;
//           let path = entry.path();
//           rm(&path, root_dir, exists)?;
//         }
//       }
//     }
//   } else if path.is_file() {
//     if let Ok(rel) = path.strip_prefix(root_dir) {
//       if !exists.contains(&rel.to_path_buf()) {
//         fs::remove_file(path)?;
//       }
//     }
//   }
//   Ok(())
// }
