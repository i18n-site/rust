use std::{
  collections::{HashMap, HashSet},
  fs,
  io::Write,
  path::PathBuf,
};

use aok::{Result, OK};
use ifs::is_hidden;
use prost::Message;
use walkdir::WalkDir;

use crate::{api, Site};

const EMPTY: String = String::new();
pub type LangDirName = (u32, String, &'static str);

#[allow(async_fn_in_trait)]
pub trait Upload {
  async fn run(
    site: Site,
    dir: PathBuf,
    mut lang_path: Vec<LangDirName>,
    upload_ext: Vec<String>,
    nav_li: Vec<String>,
  ) -> Result<()> {
    lang_path.sort_by(|a, b| a.0.cmp(&b.0));
    let mut url_set = HashSet::new();
    let lang_path_len = lang_path.len();
    let mut lang_url_li = Vec::with_capacity(lang_path_len);
    for i in lang_path {
      let dir = dir.join(&i.1);
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
      lang_url_li.push((i, lang_url_set));
    }
    let mut url_li = url_set.into_iter().collect::<Vec<_>>();
    url_li.sort();
    let site = api::Site {
      host: site.host,
      render_li: site.render_li,
      nav_li: site.nav_li,
      lang_li: Self::upload_lang(&dir, nav_li, &url_li, lang_url_li).await?,
      url_li,
    };
    Self::upload_site(dir, site).await
  }

  async fn upload_site(dir: PathBuf, site: api::Site) -> Result<()>;

  async fn upload_lang(
    dir: &PathBuf,
    nav_li: Vec<String>,
    url_li: &[String],
    lang_url_li: Vec<(LangDirName, HashSet<String>)>,
  ) -> Result<Vec<api::Lang>>;
}

pub struct NoUpload;

impl Upload for NoUpload {
  async fn upload_site(dir: PathBuf, site: api::Site) -> Result<()> {
    let site_bin = site.encode_to_vec();
    let public = dir.join("public");
    let fp = "site";
    ifs::w(public.join(fp))?.write_all(&site_bin)?;
    ifs::w(public.join(".v"))?.write_all(fp.as_bytes())?;
    OK
  }

  async fn upload_lang(
    dir: &PathBuf,
    nav_li: Vec<String>,
    url_li: &[String],
    lang_url_li: Vec<(LangDirName, HashSet<String>)>,
  ) -> Result<Vec<api::Lang>> {
    let mut r = Vec::with_capacity(lang_url_li.len());
    for (lang_dir_name, url_set) in lang_url_li {
      let nav_i18n = if let Ok(yml) = xerr::ok!(ifs::r(dir.join(&lang_dir_name.1).join("i18n.yml")))
        && let Ok(m) = xerr::ok!(serde_yaml::from_slice::<HashMap<String, String>>(&yml[..]))
      {
        m
      } else {
        Default::default()
      };
      let site_lang = api::SiteLang {
        i18n_li: nav_li
          .iter()
          .map(|nav| nav_i18n.get(nav).unwrap_or(&EMPTY).to_owned())
          .collect(),
        url_v_li: url_li
          .iter()
          .map(|url| {
            if url_set.contains(url) {
              format!("{}/{url}", lang_dir_name.1)
            } else {
              Default::default()
            }
          })
          .collect(),
      };
      dbg!(site_lang);
    }
    Ok(r)
  }
}

pub struct S3;

impl Upload for S3 {
  async fn upload_site(dir: PathBuf, site: api::Site) -> Result<()> {
    let site_bin = site.encode_to_vec();
    OK
  }

  async fn upload_lang(
    dir: &PathBuf,
    nav_li: Vec<String>,
    url_li: &[String],
    lang_url_li: Vec<(LangDirName, HashSet<String>)>,
  ) -> Result<Vec<api::Lang>> {
    todo!()
  }
}
