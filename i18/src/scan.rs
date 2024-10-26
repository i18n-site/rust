use std::path::{Path, PathBuf};

use aok::Result;
use globset::GlobSet;
use gxhash::{HashMap, HashMapExt, HashSet};
use i18_conf::I18nConf;
use i18_hash::LangLi;
use ifs::{is_dir, unix_path};
use lang::{Lang, LANG_CODE};
pub use walkdir::WalkDir;

use crate::RelFt;

#[derive(Debug)]
pub struct Scan {
  pub root: PathBuf,
  pub lang_prefix_rel: HashMap<Lang, HashMap<String, HashSet<String>>>,
  pub rel_ft: Vec<RelFt>,
}

pub fn rel_path(root: &Path, path: &Path) -> Result<String> {
  let r = path.strip_prefix(root)?;
  Ok(unix_path(r.display().to_string()))
}

pub fn join(prefix: &str, rel: &str) -> String {
  if prefix.is_empty() {
    rel.into()
  } else if rel.is_empty() {
    prefix.into()
  } else {
    format!("{}/{}", prefix, rel)
  }
}

impl Scan {
  pub fn rel_set(&self) -> HashSet<String> {
    let mut r = HashSet::default();
    for m in self.lang_prefix_rel.values() {
      for (prefix, rel_li) in m {
        for rel in rel_li {
          r.insert(join(prefix, rel));
        }
      }
    }
    r
  }

  pub fn lang_rel_li_for_tran(&self) -> Vec<(String, LangLi)> {
    self
      .lang_rel_li()
      .into_iter()
      .filter(|i| {
        let path = &i.0;
        if let Some(p) = path.rsplit('.').next() {
          return ["md", "yml"].contains(&p);
        }
        false
      })
      .collect::<Vec<_>>()
  }

  pub fn lang_rel_li(&self) -> Vec<(String, LangLi)> {
    fn push(
      li: &mut Vec<(String, LangLi)>,
      prefix: &str,
      ft: &ft::FromTo,
      lang_prefix_rel: &HashMap<Lang, HashMap<String, HashSet<String>>>,
    ) {
      if let Some(src) = ft.root() {
        if let Some(prefix_rel) = lang_prefix_rel.get(&src) {
          if let Some(rel_li) = prefix_rel.get(prefix) {
            let lang_li = ft.all_lang_li();
            for lang in lang_li {
              let mut to_lang_li = ft
                .to_li(lang)
                .unwrap_or_default()
                .into_iter()
                .collect::<Vec<_>>();

              to_lang_li.sort();

              let to_lang_li = LangLi(to_lang_li);

              for rel in rel_li {
                let fp = join(prefix, rel);
                li.push((
                  format!("{}/{}", LANG_CODE[lang as usize], fp),
                  to_lang_li.clone(),
                ));
              }
            }
          }
        }
      }
    }
    let mut li = Vec::new();
    for (prefix, ft) in &self.rel_ft {
      push(&mut li, prefix, ft, &self.lang_prefix_rel);
    }
    li
  }

  pub fn new(root: impl Into<PathBuf>, conf: &I18nConf, ignore: &GlobSet) -> Self {
    let rel_ft = crate::conf_from_to(conf);
    fn push(
      i: Result<walkdir::DirEntry, walkdir::Error>,
      src_root: &Path,
      lang_rel: &mut HashMap<String, HashSet<String>>,
      src_prefix_li: &[&String],
      ignore: &GlobSet,
    ) {
      if let Ok(i) = xerr::ok!(i) {
        let file_type = i.file_type();
        let is_file = file_type.is_file();
        if is_file {
          if let Ok(rel) = rel_path(src_root, i.path()) {
            if ignore.is_match("/".to_owned() + &rel) {
              return;
            }
            for prefix in src_prefix_li {
              if let Some(i) = rel.strip_prefix(prefix.as_str()) {
                lang_rel.entry((*prefix).into()).or_default().insert(
                  if i.starts_with("/") && i.len() > 1 {
                    i[1..].into()
                  } else {
                    i.into()
                  },
                );
                break;
              }
            }
          }
        }
      }
    }
    let root = root.into();

    let mut prefix_lang = Vec::new();
    let mut lang_prefix = HashMap::new();
    let mut all_lang = HashSet::default();

    for (prefix, ft) in &rel_ft {
      if let Some(src) = ft.root() {
        prefix_lang.push((prefix, src));
        lang_prefix.entry(src).or_insert_with(Vec::new).push(prefix);
        all_lang.insert(src);
      }
    }

    let mut lang_prefix_rel = HashMap::new();

    for src in all_lang {
      let src_root = root.join(LANG_CODE[src as usize]);
      if Some(true) == is_dir(&src_root) {
        let lang_rel = lang_prefix_rel.entry(src).or_insert_with(HashMap::new);
        let src_prefix_li = lang_prefix.remove(&src).unwrap();
        for i in WalkDir::new(&src_root).into_iter().filter_entry(|i| {
          if i.file_type().is_dir() {
            return true;
          }
          let path = i.path();
          if let Ok(rel) = rel_path(&src_root, path) {
            for (prefix, lang) in &prefix_lang {
              if rel.starts_with(prefix.as_str()) {
                if *lang == src {
                  return true;
                }
                break;
              }
            }
          }
          false
        }) {
          push(i, &src_root, lang_rel, &src_prefix_li, ignore);
        }
      }
    }

    Scan {
      root,
      lang_prefix_rel,
      rel_ft,
    }
  }
}
