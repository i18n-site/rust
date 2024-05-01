use std::path::PathBuf;

use daachorse::{DoubleArrayAhoCorasick, DoubleArrayAhoCorasickBuilder, MatchKind};
use walkdir::WalkDir;

pub struct VDir {
  pub ac: DoubleArrayAhoCorasick<usize>,
}

impl VDir {
  pub fn find<'a>(&self, path: &'a str) -> Option<&'a str> {
    let mut it = self.ac.leftmost_find_iter(path);

    if let Some(m) = it.next() {
      if m.start() == 0 {
        let end = m.end();
        if end != path.len() && !path[end..].starts_with('/') {
          return None;
        }

        return Some(&path[..end]);
      }
    }

    None
  }

  pub fn new(root: &PathBuf) -> Self {
    let mut li = vec![];

    let dir_v = root.join("v");

    if dir_v.exists() {
      for entry in WalkDir::new(&dir_v).into_iter().filter_entry(dot_hide::not) {
        if let Ok(entry) = xerr::ok!(entry) {
          if entry.file_type().is_file() {
            if let Some(file_name) = entry.file_name().to_str() {
              if file_name == "@" {
                let dir = entry.path().parent().unwrap().strip_prefix(&dir_v).unwrap();
                if let Some(dir) = dir.to_str() {
                  if dir == "@" {
                    continue;
                  }
                  let dir = dir.replace('\\', "/");
                  li.push(dir);
                  // dbg!(file_name, dir);
                }
              }
            }
          }
        }
      }
    }

    li.sort_by_key(|i| std::cmp::Reverse(i.len()));

    dbg!(&li);
    Self {
      ac: DoubleArrayAhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostLongest)
        .build(&li)
        .unwrap(),
    }
  }
}
