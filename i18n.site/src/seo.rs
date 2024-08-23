use std::{
  fs::File,
  io::{BufRead, BufReader},
  path::{Path, PathBuf},
};

use aok::{Null, Result, OK};
use futures::{stream, stream::StreamExt};
use globset::GlobSet;
use gxhash::{HashMap, HashSet};
use i18::DOT_I18N;
use ifs::unix_path;
use lang::{Lang, LANG_CODE};
use walkdir::WalkDir;
use ytree::lang::RelLangSet;

pub trait Seo {
  fn init(root: &Path, name: &str, host: String) -> Result<Self>
  where
    Self: Sized;

  async fn put(&self, rel: &str) -> Result<bool>;

  async fn end(&self, rel_lang_set: &RelLangSet) -> Null;
}

pub fn md_url<'a>(rel: &'a str) -> &'a str {
  if let Some(rel) = rel.strip_suffix(".md") {
    if rel.ends_with("/README") {
      &rel[..rel.len() - 7]
    } else {
      rel
    }
  } else {
    rel
  }
}

pub struct Fs {
  pub out: PathBuf,
  pub root: PathBuf,
  pub host: String,
}

impl Seo for Fs {
  async fn end(&self, rel_lang_set: &RelLangSet) -> Null {
    dbg!(&self.root, &self.host, &self.out);
    for (rel, lang_set) in rel_lang_set.0.iter() {
      let rel = md_url(rel);
      let url = format!("https://{}/{}", self.host, rel);
      dbg!(url);
      for lang in lang_set {
        let lang = LANG_CODE[lang as usize];
        let url = format!("https://{}/{lang}/{rel}.htm", self.host);
        dbg!(url);
      }
    }
    // for i in rel_lang_set {
    //   dbg!(i);
    // }
    OK
  }

  fn init(root: &Path, name: &str, host: String) -> Result<Self> {
    let out = root.join("out").join(name).join("htm");
    Ok(Self {
      host,
      out: out.into(),
      root: root.into(),
    })
  }

  async fn put(&self, rel: &str) -> Result<bool> {
    if let Some(htm) = md2htm(&self.root.join(rel))? {
      let url = md_url(rel);
      let to = format!("{}.htm", url);
      ifs::wbin(&self.out.join(to), htm.as_bytes())?;
      return Ok(true);
    }
    Ok(false)
  }
}

pub fn md2htm(fp: &Path) -> Result<Option<String>> {
  let md = std::fs::read_to_string(fp)?;
  {
    use std::io::Cursor;
    let c = Cursor::new(&md);
    let mut iter = c.lines();
    let mut n = 0;
    'out: loop {
      while let Some(Ok(i)) = iter.next() {
        if !i.trim().is_empty() {
          n += 1;
          if n > 1 {
            break 'out;
          }
        }
      }
      return Ok(None);
    }
  }

  let mut opt = markdown::Options::gfm();
  opt.compile.allow_dangerous_html = true;
  opt.compile.allow_dangerous_protocol = true;
  if let Ok(htm) = xerr::ok!(markdown::to_html_with_options(&md, &opt)) {
    let htm = htm.replace(">\n<", "><");
    let htm = htm.trim_end();
    return Ok(Some(format!(
      r#"<!doctypehtml><head><meta charset=UTF-8><script src=//registry.npmmirror.com/18x/latest/files/seo.js></script></head><body>{htm}</body>"#
    )));
  };
  Ok(None)
}

async fn upload(
  upload: &impl Seo,
  root: &Path,
  lang_li: &[Lang],
  changed: &HashSet<String>,
  mut exist: RelLangSet,
  ignore: &GlobSet,
) -> Result<String> {
  let (mut to_insert, mut to_remove) = {
    let exist = &exist;
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
                          if let Some(rel) = path.to_str() {
                            let rel = unix_path(rel);
                            let fp = format!("{lang_en}/{rel}");
                            if ignore.is_match(format!("/{fp}")) {
                              return None;
                            }
                            if !changed.contains(&fp) {
                              if exist.contains(lang, &rel) {
                                return None;
                              }
                            }
                            return Some((fp, lang, rel));
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
        .map(|(fp, lang, rel)| async move { (upload.put(&fp).await, lang, rel) }),
    )
    .buffer_unordered(6);
    let mut to_insert = vec![];
    let mut to_remove = vec![];
    while let Some((r, lang, rel)) = iter.next().await {
      if let Ok(is_put) = xerr::ok!(r) {
        if is_put {
          to_insert.push((lang, rel));
          continue;
        }
      }
      to_remove.push((lang, rel));
    }
    (to_insert, to_remove)
  };

  while let Some((lang, rel)) = to_insert.pop() {
    exist.insert(lang, rel);
  }
  while let Some((lang, rel)) = to_remove.pop() {
    exist.remove(lang, rel);
  }

  upload.end(&exist).await?;
  Ok(exist.dumps())
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
      let m = {
        let host = host.clone();
        if action == "fs" {
          Fs::init(root, name, host)
        } else {
          eprintln!("seo {name} {host} {action} not support");
          continue;
        }
      };

      match m {
        Err(e) => {
          eprintln!("seo {name} {host} {action} {e}");
          continue;
        }
        Ok(m) => {
          let seo_fp = root.join(DOT_I18N).join("seo").join(host).join(action);
          let exist = if seo_fp.exists() {
            let reader = BufReader::new(File::open(&seo_fp)?);
            ytree::lang::loads(reader.lines().filter_map(|i| i.ok()))
          } else {
            Default::default()
          };
          if let Ok(yml) = xerr::ok!(
            upload(
              &m,
              root,
              &lang_li,
              changed,
              exist.rel_lang_set(root)?,
              ignore,
            )
            .await
          ) {
            ifs::wbin(seo_fp, yml)?;
          }
        }
      }
    }
  }

  OK
}
