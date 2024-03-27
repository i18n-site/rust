use std::{collections::HashSet, fs, path::PathBuf};

use aok::{Result, OK};
use ifs::is_hidden;
use walkdir::WalkDir;

use crate::{api, Site};

#[allow(async_fn_in_trait)]
pub trait Upload {
  async fn run(
    site: Site,
    dir: PathBuf,
    mut lang_path: Vec<(u32, String, &'static str)>,
    upload_ext: Vec<String>,
    nav_li: Vec<String>,
  ) -> Result<()> {
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
    let site = api::Site {
      host: site.host,
      render_li: site.render_li,
      nav_li: site.nav_li,
      lang_li: Self::upload_lang(dir, nav_li, &url_li, lang_url_li).await?,
      url_li,
    };
    Self::upload_site(site).await
  }

  async fn upload_site(site: api::Site) -> Result<()>;

  async fn upload_lang(
    dir: PathBuf,
    nav_li: Vec<String>,
    url_li: &[String],
    lang_url_li: Vec<(u16, HashSet<String>)>,
  ) -> Result<Vec<api::Lang>>;
}

pub struct NoUpload;

impl Upload for NoUpload {
  async fn upload_site(site: api::Site) -> Result<()> {
    OK
  }
  async fn upload_lang(
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
  async fn upload_site(site: api::Site) -> Result<()> {
    OK
  }
  async fn upload_lang(
    dir: PathBuf,
    nav_li: Vec<String>,
    url_li: &[String],
    lang_url_li: Vec<(u16, HashSet<String>)>,
  ) -> Result<Vec<api::Lang>> {
    todo!()
  }
}
