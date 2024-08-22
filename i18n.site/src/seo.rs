use std::{
  collections::BTreeMap,
  fs::File,
  io::{BufRead, BufReader},
  path::{Path, PathBuf},
};

use aok::{Null, Result, OK};
use futures::{stream, stream::StreamExt};
use globset::GlobSet;
use gxhash::{HashMap, HashMapExt, HashSet, HashSetExt};
use i18::DOT_I18N;
use ifs::unix_path;
use lang::{Lang, LANG_CODE};
use walkdir::WalkDir;

// file hash

pub trait Seo {
  fn init(root: &Path, name: &str, host: &str) -> Result<Self>
  where
    Self: Sized;

  async fn put(&self, rel: &str) -> Null;
}

pub struct Fs {
  pub out: PathBuf,
  pub root: PathBuf,
}

impl Seo for Fs {
  fn init(root: &Path, name: &str, _host: &str) -> Result<Self> {
    let out = root.join("out").join(name).join("htm");
    Ok(Self {
      out: out.into(),
      root: root.into(),
    })
  }

  async fn put(&self, rel: &str) -> Null {
    let htm = md2htm(&self.root.join(rel))?;
    ifs::wbin(
      &self.out.join(format!("{}htm", &rel[..rel.len() - 2])),
      htm.as_bytes(),
    )?;
    OK
  }
}

pub fn md2htm(fp: &Path) -> Result<String> {
  let md = std::fs::read_to_string(fp)?;
  let htm = markdown::to_html(&md).replace(">\n<", "><");
  let htm = htm.trim_end();
  return Ok(format!(
    r#"<!doctypehtml><head><meta charset=UTF-8><script src=//registry.npmmirror.com/18x/latest/files/seo.js></script></head><body>{htm}</body>"#
  ));
}

async fn upload(
  upload: &impl Seo,
  root: &Path,
  lang_li: &[Lang],
  changed: &HashSet<String>,
  exist: &HashMap<String, HashSet<Lang>>,
  ignore: &GlobSet,
) {
  let mut uploaded = HashSet::new();
  let mut iter = stream::iter(
    lang_li
      .iter()
      .flat_map(|&lang| {
        let lang_en = LANG_CODE[lang as usize];
        let dir = root.join(lang_en);
        WalkDir::new(&dir)
          .into_iter()
          .filter_entry(dot_hide::not)
          .filter_map(move |entry| {
            entry.ok().and_then(|entry| {
              let path = entry.path();
              if let Some(ext) = path.extension() {
                if ext == "md" {
                  if let Ok(meta) = entry.metadata() {
                    if meta.file_type().is_file() {
                      if let Ok(path) = path.strip_prefix(&dir) {
                        if let Some(p) = path.to_str() {
                          let p = unix_path(p);
                          let rel = format!("{lang_en}/{p}");
                          if ignore.is_match(format!("/{rel}")) {
                            return None;
                          }
                          if !changed.contains(&rel) {
                            if let Some(exist) = exist.get(&rel) {
                              if exist.contains(&lang) {
                                return None;
                              }
                            }
                          }
                          return Some(rel);
                        }
                      }
                    }
                  }
                }
              }
              None
            })
          })
      })
      .map(|path| async move { (upload.put(&path).await, path) }),
  )
  .buffer_unordered(6);
  while let Some((r, path)) = iter.next().await {
    if let Ok(_) = xerr::ok!(r) {
      uploaded.insert(path);
    }
  }

  let mut rel_lang = HashMap::default();

  macro_rules! add {
    ($i:expr) => {
      let mut i = $i.split('/');
      if let Some(lang) = i.next() {
        if let Some(rel) = i.remainder() {
          if let Ok::<Lang, _>(lang) = lang.try_into() {
            rel_lang
              .entry(rel.to_owned())
              .or_insert_with(Vec::new)
              .push(lang);
          }
        }
      }
    };
  }

  for i in uploaded {
    add!(i);
  }

  for (rel, lang_li) in exist {
    let e = rel_lang.entry(rel.to_owned()).or_insert_with(Vec::new);
    for lang in lang_li {
      if !e.contains(&lang) {
        e.push(*lang);
      }
    }
  }

  let mut lang_file = std::collections::HashMap::new();
  for (rel, mut li) in rel_lang {
    li.sort();
    let bin: Vec<u8> = vb::diffe(li.into_iter().map(|i| i as u64).collect::<Vec<_>>());
    lang_file.entry(bin).or_insert_with(Vec::new).push(rel);
  }

  let mut to_write = Vec::with_capacity(lang_file.len());

  for (lang_bin, mut li) in lang_file {
    li.sort();

    to_write.push((burl::e(lang_bin)));
  }
}

pub async fn seo(
  root: &Path,
  name: &str,
  conf: &HashMap<String, String>,
  lang_li: Vec<Lang>,
  ignore: &GlobSet,
  changed: &HashSet<String>,
) -> Null {
  for (host, action) in conf {
    for action in action.split_whitespace() {
      let m = if action == "fs" {
        Fs::init(root, name, host)
      } else {
        eprintln!("seo {name} {host} {action} not support");
        continue;
      };

      match m {
        Err(e) => {
          eprintln!("seo {name} {host} {action} {e}");
          continue;
        }
        Ok(m) => {
          let seo_exist = root.join(DOT_I18N).join("seo").join(host).join(action);
          let mut exist = HashMap::default();
          if seo_exist.exists() {
            let reader = BufReader::new(File::open(seo_exist)?);
            for i in reader.lines().map_while(Result::ok) {}
          };
          upload(&m, root, &lang_li, changed, &exist, ignore).await;
        }
      }
    }
  }

  OK
}
