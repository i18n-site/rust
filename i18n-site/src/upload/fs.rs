use std::{io::Write, path::Path};

use aok::{Result, OK};
use prost::Message;

use crate::{
  api,
  upload::{LangUrlLi, UrlLiExt, LANG, PUBLIC, V},
  Upload, EMPTY,
};

#[derive(Default)]
pub struct Fs;

impl Upload for Fs {
  async fn upload_site(
    &mut self,
    channel: String,
    ver: String,
    dir: &Path,
    site: api::Site,
  ) -> Result<()> {
    let public = dir.join(PUBLIC);
    let fp = format!("{V}/{}/{}/index", &channel, &ver);
    ifs::w(public.join(".v"))?.write_all(fp.as_bytes())?;
    let site_bin = site.encode_to_vec();
    ifs::w(public.join(fp))?.write_all(&site_bin)?;
    OK
  }

  async fn upload_lang(
    &mut self,
    dir: &Path,
    ver: &str,
    nav_li: Vec<String>,
    url_li_ext: &[UrlLiExt],
    ext_url_li: LangUrlLi,
  ) -> Result<Vec<api::Lang>> {
    let mut r = Vec::with_capacity(ext_url_li.len());
    let public = dir.join(PUBLIC).join(V).join(ver).join(LANG);
    for (lang_dir_name, url_set) in ext_url_li {
      let mut url_v_li = Vec::with_capacity(url_li_ext.iter().map(|i| i.url_li.len()).sum());

      for i in url_li_ext {
        let ext = &i.ext;
        if let Some(set) = url_set.get(ext) {
          for url in &i.url_li {
            url_v_li.push(if set.contains(url) {
              format!("{}/{url}.{ext}", lang_dir_name.1)
            } else {
              EMPTY
            })
          }
        } else {
          i.url_li.iter().for_each(|_| {
            url_v_li.push(EMPTY);
          });
        }
      }

      let site_lang = crate::site_lang(&nav_li, dir, &lang_dir_name.1, url_v_li);
      let en = &lang_dir_name.1;
      ifs::w(public.join(en))?.write_all(&site_lang.encode_to_vec())?;
      r.push(api::Lang {
        en: en.into(),
        name: lang_dir_name.2.into(),
        url: format!("{V}/{ver}/{LANG}/{en}"),
      });
    }
    Ok(r)
  }
}
