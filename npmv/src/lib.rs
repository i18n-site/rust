use std::{
  fs::File,
  io::{self, BufReader},
  path::Path,
  time::Duration,
};

use aok::{Null, Result, OK};
use flate2::read::GzDecoder;
use reqwest::{Client, Response};
use serde::Deserialize;
use tar::Archive;

pub const SITE_LI: &[&str] = &[
  "registry.npmjs.org",
  "registry.npmmirror.com",
  "mirrors.cloud.tencent.com/npm",
  "mirrors.huaweicloud.com/repository/npm",
];

#[derive(Deserialize, Debug)]
pub struct Info {
  pub version: String,
}

async fn response(pkg: &str, url: &str) -> reqwest::Result<Response> {
  let client = Client::builder()
    .timeout(Duration::from_secs(120))
    .connect_timeout(Duration::from_secs(6))
    .build()?;

  let mut iter = SITE_LI.iter();
  let mut site = iter.next().unwrap();

  loop {
    match client
      .get(format!("https://{site}/{pkg}/{url}"))
      .send()
      .await
    {
      Err(e) => {
        if let Some(s) = iter.next() {
          tracing::warn!("{}: {}", site, e);
          site = s;
        } else {
          return Err(e);
        }
      }
      Ok(response) => match response.error_for_status() {
        Err(e) => {
          if let Some(s) = iter.next() {
            tracing::warn!("{}: {}", site, e);
            site = s;
          } else {
            return Err(e);
          }
        }
        Ok(response) => return Ok(response),
      },
    }
  }
}

pub async fn latest(name: impl AsRef<str>) -> Result<String> {
  let bin = response(name.as_ref(), "latest").await?.bytes().await?;
  let info: Info = sonic_rs::from_slice(&bin)?;
  Ok(info.version)
}

pub async fn tgz(name: impl AsRef<str>, ver: impl AsRef<str>, out: impl AsRef<Path>) -> Null {
  let out = out.as_ref();
  let name = name.as_ref();
  let bytes = response(
    name,
    &format!(
      "-/{}-{}.tgz",
      name.split("/").last().unwrap_or(""),
      ver.as_ref()
    ),
  )
  .await?
  .bytes()
  .await?;

  let tar_gz = GzDecoder::new(&bytes[..]);
  let mut archive = Archive::new(tar_gz);

  if let Ok(meta) = std::fs::metadata(out) {
    if meta.is_dir() {
      std::fs::remove_dir_all(out)?;
    } else {
      std::fs::remove_file(out)?;
    }
  }

  for entry in archive.entries()? {
    let mut entry = entry?;
    if let Ok(path) = entry.path() {
      let path = path.display().to_string();
      if let Some(path) = path.strip_prefix("package") {
        let path = &path[1..];
        let fp = out.join(path);

        if let Err(err) = entry.unpack(&fp) {
          let kind = err.kind();
          if kind == io::ErrorKind::NotFound {
            if let Some(parent) = fp.parent() {
              std::fs::create_dir_all(parent)?;
              entry.unpack(&fp)?;
            }
          } else {
            tracing::error!("{}: {}", kind, err);
          }
        }
      }
    }
  }

  OK
}
