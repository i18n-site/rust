use std::{
  collections::HashMap,
  path::{Path, PathBuf},
};

use aok::{Result, OK};
use map_await::{MapAwait, StreamExt};
use mysql_macro::{args, e, last_id, q, q01};
use prost::Message;
use s3_put::IntoByteStream;
use tracing::info;

use crate::{
  api,
  upload::{LangUrlLi, UrlLiExt},
  Upload, EMPTY,
};

pub struct S3 {
  s3: s3_put::S3,
}

impl Default for S3 {
  fn default() -> Self {
    Self {
      s3: s3_put::S3::new(),
    }
  }
}

pub const TEXT_JS: &str = "text/js";

genv::def!(S3_JOB:usize | 16);

pub fn urle(n: u64) -> String {
  burl::e(intbin::u64_bin(n))
}

pub async fn hash_upload_bin(
  s3: &mut s3_put::S3,
  name: &str,
  bin: Vec<u8>,
) -> Result<(u64, String)> {
  let s3 = s3.get();
  let hash = *blake3::hash(&bin).as_bytes();
  let id_upload: Option<(u64, bool)> = q01!("SELECT id,uploaded FROM dist WHERE hash=?", hash);
  let (id, upload) = if let Some(id_upload) = id_upload {
    id_upload
  } else {
    (
      last_id(
        "INSERT INTO dist (name,hash) VALUES (?,?)",
        vec![name.into(), hash.into()],
      )
      .await?,
      false,
    )
  };
  let url = urle(id);
  if !upload {
    s3.put(&url, TEXT_JS, bin).await?;
    e!("UPDATE dist SET uploaded=1 WHERE id=?", id);
  }
  Ok((id, url))
}

pub async fn hash_upload<T: IntoByteStream + Send>(
  s3: &mut s3_put::S3,
  mut hashmap: HashMap<[u8; 32], String>,
  file_path: impl Fn(String) -> T + Send + Sync + 'static + Clone,
) -> Result<
  HashMap<Vec<u8>, u64>, //hash_id
> {
  let hashmap_len = hashmap.len();
  let mut hash_id = HashMap::with_capacity(hashmap_len);
  let mut to_upload_id_fp = HashMap::new();

  if hashmap_len > 0 {
    loop {
      let (hash_vec, args) = args(hashmap.keys());
      let hash_id_uploaded: Vec<(u64, Vec<u8>, bool)> = q(
        format!("SELECT id,hash,uploaded FROM dist WHERE hash IN ({args})"),
        hash_vec,
      )
      .await?;
      for i in hash_id_uploaded {
        if let Some(fp) = hashmap.remove(&i.1[..32]) {
          hash_id.insert(i.1.clone(), i.0);
          if !i.2 {
            to_upload_id_fp.insert(i.0, fp);
          }
        }
      }
      let len = hashmap.len();
      if len == 0 {
        break;
      }
      let mut args = String::new();
      let mut hash_name_vec = Vec::with_capacity(len * 2);
      hashmap.iter().for_each(|i| {
        args.push_str("(?,?),");
        hash_name_vec.push(i.0.into());
        hash_name_vec.push(i.1.into());
      });
      args.truncate(args.len() - 1);
      e(
        format!(
          "INSERT INTO dist (hash,name) VALUES {args} ON DUPLICATE KEY UPDATE name=VALUES(name)"
        ),
        hash_name_vec,
      )
      .await?;
    }
  }
  if !to_upload_id_fp.is_empty() {
    let (hash_vec, args) = args(to_upload_id_fp.keys());

    let s3 = s3.get();
    let mut iter = to_upload_id_fp
      .into_iter()
      .map_unordered(S3_JOB(), move |(id, fp)| {
        let s3 = s3.clone();
        let mime = if let Some(pos) = fp.rfind('.') {
          let ext = &fp[pos + 1..];
          if ["md", "txt", "c", "cpp", "rs"].contains(&ext) {
            // for compression : https://developers.cloudflare.com/speed/optimization/content/brotli/content-compression/
            TEXT_JS
          } else {
            mime_guess::from_ext(ext).first_raw().unwrap_or(TEXT_JS)
          }
        } else {
          TEXT_JS
        };
        let url = urle(id);
        let file_path = file_path.clone();
        async move {
          info!("⇧ {fp} → {url}");
          let path = file_path(fp);
          s3.put(&url, mime, path).await?;
          OK
        }
      });

    while let Some(r) = iter.next().await {
      r?;
    }

    e(
      format!("UPDATE dist SET uploaded=1 WHERE id IN ({args})"),
      hash_vec,
    )
    .await?;
  }
  Ok(hash_id)
}

impl Upload for S3 {
  async fn upload_site(
    &mut self,
    channel: String,
    ver: String,
    _dir: &Path,
    site: api::Site,
  ) -> Result<()> {
    let host = &site.host.to_lowercase();

    let site_id: Option<u64> = q01!("SELECT id FROM site WHERE name=?", &host);
    let site_id = if let Some(site_id) = site_id {
      site_id
    } else {
      last_id(
        "INSERT INTO site (name,uid) VALUES (?,0)",
        vec![host.into()],
      )
      .await?
    };
    let (dist_id, dist_url) = hash_upload_bin(&mut self.s3, host, site.encode_to_vec()).await?;

    let exist: Option<u64> = q01!(
      "SELECT distId FROM siteV WHERE siteId=? AND channel=? AND v=? ORDER BY ts DESC LIMIT 1",
      site_id,
      &channel,
      &ver
    );

    if exist == Some(dist_id) {
      return OK;
    }

    let url = format!("{}/{}/.v", host, &channel);
    self
      .s3
      .get()
      .put(&url, TEXT_JS, dist_url.as_bytes().to_vec())
      .await?;

    e!(
      "INSERT INTO siteV (siteId,channel,v,distId,ts) VALUES (?,?,?,?,UNIX_TIMESTAMP()) ON DUPLICATE KEY UPDATE ts=VALUES(ts)",
      site_id,
      &channel,
      &ver,
      dist_id
    );
    // let fp = dir.join(&site.name);
    // let s3 = s3_put::S3::new();
    // s3.put(&burl::e(intbin::u64_bin(id)), TEXT_JS, fp).await?;

    OK
  }

  async fn upload_lang(
    &mut self,
    dir: &Path,
    _ver: &str,
    nav_li: Vec<String>,
    url_li_ext: &[UrlLiExt],
    ext_url_li: LangUrlLi,
  ) -> Result<Vec<api::Lang>> {
    let s3 = &mut self.s3;
    let ext_url_li_len = ext_url_li.len();
    let mut m = HashMap::with_capacity(ext_url_li_len);
    let mut hashmap = HashMap::new();
    for (lang_dir_name, url_set) in &ext_url_li {
      let mut map = HashMap::with_capacity(url_li_ext.len());
      for (ext, li) in url_set {
        let mut extmap = HashMap::with_capacity(li.len());
        for name in li {
          let rel_fp = format!("{}/{name}.{ext}", &lang_dir_name.1);
          let fp = dir.join(&rel_fp);
          let hash = *blake3::hash(&ifs::r(&fp)?).as_bytes();
          extmap.insert(name.clone(), hash);
          hashmap.insert(hash, rel_fp);
        }
        map.insert(ext, extmap);
      }
      m.insert(lang_dir_name.0, map);
    }

    let hash_id = {
      let dir: PathBuf = dir.into();
      hash_upload(s3, hashmap, move |fp| dir.join(fp)).await?
    };

    let mut r = Vec::with_capacity(ext_url_li_len);
    let mut hashmap = HashMap::with_capacity(ext_url_li_len);
    let mut name_bin = HashMap::with_capacity(ext_url_li_len);
    for (lang_dir_name, url_set) in &ext_url_li {
      let mut url_v_li = Vec::with_capacity(url_li_ext.iter().map(|i| i.url_li.len()).sum());
      let map = m.get(&lang_dir_name.0).unwrap();
      for i in url_li_ext {
        let ext = &i.ext;
        let map = map.get(&ext).unwrap();
        if let Some(set) = url_set.get(ext) {
          for url in &i.url_li {
            url_v_li.push(if set.contains(url) {
              let hash = map.get(url).unwrap();
              let id = hash_id.get(&hash[..32]).unwrap();
              urle(*id)
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
      let bin = crate::site_lang(&nav_li, dir, &lang_dir_name.1, url_v_li).encode_to_vec();
      let hash = *blake3::hash(&bin).as_bytes();
      let name = lang_dir_name.1.to_owned();
      name_bin.insert(name.clone(), bin);
      hashmap.insert(hash, name);
      r.push((lang_dir_name.1.clone(), lang_dir_name.2.into(), hash));
    }
    let hash_id = hash_upload(s3, hashmap, move |i| name_bin.get(&i).unwrap().clone()).await?;
    Ok(
      r.into_iter()
        .map(|(en, name, hash)| api::Lang {
          en,
          name,
          url: urle(*hash_id.get(&hash[..32]).unwrap()),
        })
        .collect(),
    )
  }
}
