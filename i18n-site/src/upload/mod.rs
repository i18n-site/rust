mod fs;
mod s3;

use std::{
  collections::{HashMap, HashSet},
  fs::symlink_metadata,
  path::{Path, PathBuf},
};

use aok::Result;
pub use fs::Fs;
use ifs::is_hidden;
pub use s3::S3;
use walkdir::WalkDir;

pub const V: &str = "v";
pub const LANG: &str = "lang";
pub const PUBLIC: &str = "public";

use crate::{
  api::{self, UrlLiExt},
  Site,
};

pub type LangDirName = (
  u32,
  String,       // dir
  &'static str, //name
);

pub type ExtUrlLi = HashMap<String, HashSet<String>>;
pub type LangUrlLi = Vec<(LangDirName, ExtUrlLi)>;

#[allow(async_fn_in_trait)]
pub trait Upload {
  async fn run(
    &mut self,
    site: Site,
    dir: PathBuf,
    mut lang_path: Vec<LangDirName>,
    upload_ext: Vec<String>,
    nav_li: Vec<String>,
  ) -> Result<()> {
    lang_path.sort_by(|a, b| a.0.cmp(&b.0));
    let mut ext_url_set = ExtUrlLi::new();
    let lang_path_len = lang_path.len();
    let mut lang_ext_url_li = Vec::with_capacity(lang_path_len);
    for i in lang_path {
      let dir = dir.join(&i.1);
      let mut lang_ext_url_set = ExtUrlLi::new();
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
            symlink_metadata(fp)?.is_file()
          } else {
            false
          };
          if let Some(ext) = fp.extension() {
            let ext = ext.to_str().unwrap().to_owned();
            if is_file && upload_ext.contains(&ext) {
              let rel_fp = fp.strip_prefix(&dir)?.display().to_string();
              let rel_fp = &rel_fp[..rel_fp.len() - 1 - ext.len()];

              lang_ext_url_set
                .entry(ext.clone())
                .or_default()
                .insert(rel_fp.into());
            }
          }
        }
      }
      for i in &lang_ext_url_set {
        ext_url_set
          .entry(i.0.clone())
          .or_default()
          .extend(i.1.clone());
      }
      lang_ext_url_li.push((i, lang_ext_url_set));
    }
    let mut url_li_ext = ext_url_set
      .into_iter()
      .map(|(ext, li)| {
        let mut li: Vec<_> = li.into_iter().collect();
        li.sort();
        (ext, li)
      })
      .collect::<Vec<_>>();
    url_li_ext.sort_by(|a, b| a.0.cmp(&b.0));

    let url_li_ext = url_li_ext
      .into_iter()
      .map(|(ext, url_li)| UrlLiExt {
        url_li,
        ext: ext.clone(),
      })
      .collect::<Vec<_>>();

    let api_site = api::Site {
      lang_li: self
        .upload_lang(&dir, &site.ver, nav_li, &url_li_ext, lang_ext_url_li)
        .await?,
      host: site.host,
      nav_li: site.nav_li,
      route_li: site.route_li,
      url_li_ext,
    };
    self
      .upload_site(site.channel, site.ver, &dir, api_site)
      .await
  }

  async fn upload_site(
    &mut self,
    channel: String,
    ver: String,
    dir: &Path,
    site: api::Site,
  ) -> Result<()>;

  async fn upload_lang(
    &mut self,
    dir: &Path,
    ver: &str,
    nav_li: Vec<String>,
    url_li_ext: &[UrlLiExt],
    ext_url_li: LangUrlLi,
  ) -> Result<Vec<api::Lang>>;
}
