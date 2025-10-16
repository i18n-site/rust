use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::INDEX_PUG;

#[derive(Debug)]
pub struct Scan {
  pub css_li: Vec<PathBuf>,
  pub pug_li: Vec<PathBuf>,
}

/// scan for pug and css
impl Scan {
  pub fn new(htm: &Path) -> Self {
    let mut css_li = vec![];
    let mut pug_li = vec![];

    for entry in WalkDir::new(htm).into_iter().filter_entry(dot_hide::not) {
      if let Ok(entry) = xerr::ok!(entry) {
        if entry.file_type().is_file() {
          let path = entry.path();
          if let Some(ext) = path.extension().map(|i| i.to_str().unwrap()) {
            let filename = path.file_name().unwrap().to_str().unwrap();
            if filename.starts_with("_") {
              continue;
            }
            let rel = path.strip_prefix(htm).unwrap().to_path_buf();
            match ext {
              "css" => {
                css_li.push(rel);
              }
              "pug" => {
                if rel.to_str().unwrap() != INDEX_PUG {
                  pug_li.push(rel);
                }
              }
              _ => {}
            }
          }
        }
      }
    }
    pug_li.sort();
    css_li.sort();

    Self { css_li, pug_li }
  }
}
