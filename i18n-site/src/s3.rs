use std::{
  collections::HashMap,
  path::{Path, PathBuf},
};

use aok::{Null, OK};
use dashmap::DashMap;
use ifs::b3;
use lang::{Lang, LANG_CODE};
use prost::Message;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tokio_postgres::error::SqlState;
use tracing::info;

use crate::{
  api, mime::TEXT_JS, s3_hash_id, s3_upload, s3_upload::urle, Conf, HashLen, Pg, Upload, INIT_SQL,
};

#[derive(Debug)]
pub struct S3<'a> {
  pub lang_rel: HashMap<Lang, Vec<String>>,
  pub all_lang: &'a [Lang],
  pub id: String,
}

impl<'a> S3<'a> {
  pub fn new(conf: &Conf, all_lang: &'a [Lang]) -> Self {
    Self {
      lang_rel: HashMap::new(),
      id: conf.id.clone(),
      all_lang,
    }
  }

  async fn site_id_upload_id(&self, pg: &Pg) -> Result<(i64, i64), tokio_postgres::Error> {
    loop {
      match pg
        .query_one("SELECT * FROM site_id_upload_id($1)", &[&self.id])
        .await
      {
        Ok(row) => {
          let id: i64 = row.get(0);
          let upload_id: i64 = row.get(1);
          return Ok((id, upload_id));
        }
        Err(err) => {
          if err.code() == Some(&SqlState::UNDEFINED_FUNCTION) {
            info!("INIT DB");
            pg.simple_query(INIT_SQL).await?;
            info!("INIT DONE");
            continue;
          }
          return Err(err);
        }
      }
    }
  }
}

impl<'a> Upload for S3<'a> {
  fn add(&mut self, lang: Lang, rel: &str) {
    self.lang_rel.entry(lang).or_default().push(rel.to_string());
  }

  async fn upload(
    self,
    mut site: api::Site,
    root: &Path,
    vlang_li: Vec<api::Vlang>,
    lang_bin_li: Vec<Vec<u8>>,
  ) -> Null {
    println!("S3 UPLOAD BEGIN");
    let pg = Pg::from_env().await?;
    let (site_id, upload_id) = self.site_id_upload_id(&pg).await?;

    let hash_fp = DashMap::new();

    let li = self
      .lang_rel
      .into_par_iter()
      .map(|(lang, rel_li)| {
        let mut map = HashMap::with_capacity(rel_li.len());
        let dir = root.join(LANG_CODE[lang as usize]);
        rel_li.into_iter().for_each(|rel| {
          let fp = dir.join(&rel);
          if let Ok(hash_len) = xerr::ok!(ifs::hash_len(&fp)) {
            hash_fp.insert(hash_len, fp);
            map.insert(rel.to_owned(), hash_len);
          }
        });
        (lang, map)
      })
      .collect::<HashMap<_, _>>();

    let (lang_bin_id_li, _) = tokio::try_join!(
      s3_hash_id(
        &pg,
        site_id,
        lang_bin_li
          .into_iter()
          .zip(self.all_lang)
          .map(|(i, lang)| ((b3(&i), i.len()), i, LANG_CODE[*lang as usize]))
          .collect::<Vec<_>>(),
      ),
      upload_vtab_vlang(
        &pg,
        &mut site,
        site_id,
        root,
        hash_fp,
        vlang_li,
        self.all_lang,
        li,
      )
    )?;

    site.lang_li = site
      .lang_li
      .into_iter()
      .zip(lang_bin_id_li)
      .map(|(mut i, id)| {
        i.url = urle(id as _);
        i
      })
      .collect();

    let bin = site.encode_to_vec();
    let site_v = format!("{}/.v", site.id);
    let site_v_id = s3_hash_id(&pg, site_id, [((b3(&bin), bin.len()), bin, site.id)]).await?[0];

    if site_v_id != upload_id {
      s3_upload::S3
        .put(site_v, TEXT_JS, urle(site_v_id as _).as_bytes())
        .await?;
      pg.execute(
        "UPDATE site SET upload_id=$1 WHERE id=$2",
        &[&site_v_id, &site_id],
      )
      .await?;
    } else {
      info!("same as prev upload");
    }
    OK
  }
}

pub async fn upload_vtab_vlang(
  pg: &Pg,
  site: &mut api::Site,
  site_id: i64,
  root: &Path,
  hash_fp: DashMap<HashLen, PathBuf>,
  mut vlang_li: Vec<api::Vlang>,
  all_lang: &[Lang],
  lang_fp_hash: HashMap<Lang, HashMap<String, HashLen>>,
) -> Null {
  let mut hash_li = Vec::with_capacity(hash_fp.len());
  let hash_fp_li = hash_fp
    .into_iter()
    .map(|i| {
      hash_li.push(i.0);
      let name = i
        .1
        .strip_prefix(root)
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_owned();
      (i.0, i.1, name)
    })
    .collect::<Vec<_>>();
  let hash_id_li = s3_hash_id(pg, site_id, hash_fp_li).await?;
  let hash_id_map: HashMap<HashLen, i64> = HashMap::from_iter(hash_li.into_iter().zip(hash_id_li));

  for (vtab, ref mut vlang) in site.vtab_li.iter().zip(vlang_li.iter_mut()) {
    let mut prefix = vtab.prefix.to_owned();

    if !prefix.is_empty() {
      prefix.push('/');
    };

    let mut data_li = Vec::new();

    for lang in all_lang {
      let mut url_li = vec![];
      for li in vlang.ext_url_li.iter() {
        let ext = &li.ext;
        let suffix = if ext.is_empty() {
          ext.to_owned()
        } else {
          format!(".{ext}")
        };

        // 按 Vlang.ext_url_li 的顺序上传
        let mut rel_li = Vec::new();

        for i in &li.url_li {
          let rel = format!(
            "{prefix}{}{suffix}",
            if i.is_empty() && ext == "md" {
              "README"
            } else {
              i
            }
          );
          rel_li.push(rel);
        }

        if let Some(m) = lang_fp_hash.get(lang) {
          for i in &rel_li {
            url_li.push(if let Some(hash) = m.get(i) {
              urle(*hash_id_map.get(hash).unwrap() as _)
            } else {
              "".into()
            });
          }
        }
      }

      let url_li = url_li.join(" ");
      let url_li = url_li.as_bytes();
      let hash = ifs::b3(url_li);
      let hash_len = (hash, url_li.len());
      data_li.push((
        hash_len,
        url_li.to_owned(),
        format!("{}/{}", LANG_CODE[*lang as usize], prefix),
      ));
    }

    vlang.lang_url_li = s3_hash_id(pg, site_id, data_li)
      .await?
      .into_iter()
      .map(|i| urle(i as _))
      .collect::<Vec<_>>();
  }

  let vlang_id_li = s3_hash_id(
    pg,
    site_id,
    site
      .vtab_li
      .iter()
      .zip(vlang_li.iter())
      .map(|(vtab, vlang)| {
        let bin = vlang.encode_to_vec();
        let p = &vtab.prefix;
        let name = format!(
          "{}{}",
          if p.is_empty() {
            p.into()
          } else {
            p.clone() + " "
          },
          vtab.ver_li[0]
        );
        ((b3(&bin), bin.len()), bin, name)
      })
      .collect::<Vec<_>>(),
  )
  .await?;

  let len = site.vtab_li.len();
  let mut prefix_li = Vec::with_capacity(len);
  let mut ver_li = Vec::with_capacity(len);

  for vtab in &site.vtab_li {
    prefix_li.push(vtab.prefix.clone());
    ver_li.push(vtab.ver_li[0].clone());
  }

  let json: String = pg
    .query_one(
      "SELECT ver_li($1,$2,$3,$4)",
      &[&site_id, &prefix_li, &ver_li, &vlang_id_li],
    )
    .await?
    .get(0);

  let li: Vec<Vec<(String, u64)>> = sonic_rs::from_str(&(format!("[{json}]")))?;

  for (ref mut vtab, ver_li) in site.vtab_li.iter_mut().zip(li) {
    let mut ver_map: HashMap<String, u64> = HashMap::from_iter(ver_li.into_iter());
    let len = vtab.ver_li.len();
    let mut ver_li = Vec::with_capacity(len);
    let mut vlang_url_li = Vec::with_capacity(len);

    for i in &vtab.ver_li {
      if let Some(id) = ver_map.remove(i) {
        ver_li.push(i.clone());
        vlang_url_li.push(urle(id));
      } else {
        eprintln!(
          ".i18n/v/{}v.yml : s3 miss version {i}",
          if vtab.prefix.is_empty() {
            "".into()
          } else {
            vtab.prefix.clone() + "/"
          }
        );
      }
    }

    vtab.ver_li = ver_li;
    vtab.vlang_url_li = vlang_url_li;
  }

  OK
}
