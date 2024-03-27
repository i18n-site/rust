use std::{collections::HashSet, fs, path::PathBuf};

use aok::Result;
use ifs::is_hidden;
use walkdir::WalkDir;

use crate::api;

#[derive(Debug, Default)]
pub struct Site {
  pub lang_li: Vec<api::Lang>,
  pub url_li: Vec<String>,
}

#[allow(async_fn_in_trait)]
pub trait Upload {
  async fn run(
    dir: PathBuf,
    mut lang_path: Vec<(u32, String, &'static str)>,
    upload_ext: Vec<String>,
    nav_li: Vec<String>,
  ) -> Result<Site> {
    lang_path.sort_by(|a, b| a.0.cmp(&b.0));
    let mut url_set = HashSet::new();
    let lang_path_len = lang_path.len();
    let mut lang_url_li = Vec::with_capacity(lang_path_len);
    for i in lang_path {
      let lang = i.1;
      let dir = dir.join(&lang);
      let mut lang_url_set = HashSet::new();
      for entry in WalkDir::new(&dir)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
      {
        if let Ok(entry) = xerr::ok!(entry) {
          let fp = entry.path();
          let file_type = entry.file_type();
          let is_file = if file_type.is_file() {
            true
          } else if file_type.is_symlink() {
            fs::symlink_metadata(fp)?.is_file()
          } else {
            false
          };
          if let Some(ext) = fp.extension() {
            let ext = ext.to_str().unwrap().to_owned();
            if is_file && upload_ext.contains(&ext) {
              let rel_fp = fp.strip_prefix(&dir)?.display().to_string();
              lang_url_set.insert(rel_fp);
            }
          }
        }
      }
      url_set.extend(lang_url_set.clone());
      lang_url_li.push((i.0 as u16, lang_url_set));
    }
    let mut url_li = url_set.into_iter().collect::<Vec<_>>();
    url_li.sort();
    // for i in lang_url_li {
    // &dir, i.0, i.1, &nav_li, &url_li
    // }
    // lang, &nav_li, lang_url_set
    // todo read nav.yml
    // api::SiteLang {
    //   nav_i18n_li,
    //   url_v_li
    // }

    Ok(Site {
      lang_li: Self::upload(dir, nav_li, &url_li, lang_url_li).await?,
      url_li,
    })
  }

  async fn upload(
    dir: PathBuf,
    nav_li: Vec<String>,
    url_li: &[String],
    lang_url_li: Vec<(u16, HashSet<String>)>,
  ) -> Result<Vec<api::Lang>>;

  // async fn upload(
  //   dir: PathBuf,
  //   lang_path: Vec<(
  //     u32,          // code
  //     String,       // en
  //     &'static str, // name
  //   )>,
  //   upload_ext: Vec<String>,
  //   nav_li: Vec<String>,
  // ) -> Result<Site>;
}

pub struct NoUpload;

impl Upload for NoUpload {
  async fn upload(
    dir: PathBuf,
    nav_li: Vec<String>,
    url_li: &[String],
    lang_url_li: Vec<(u16, HashSet<String>)>,
  ) -> Result<Vec<api::Lang>> {
    let mut r = Vec::with_capacity(lang_url_li.len());
    Ok(r)
  }
}

pub struct S3;

impl Upload for S3 {
  async fn upload(
    dir: PathBuf,
    nav_li: Vec<String>,
    url_li: &[String],
    lang_url_li: Vec<(u16, HashSet<String>)>,
  ) -> Result<Vec<api::Lang>> {
    todo!()
  }
}
