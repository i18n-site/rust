#![feature(let_chains)]

use std::{path::Path, time::Duration};

use aok::{Null, Result, OK};
use async_compression::tokio::bufread::GzipDecoder;
use async_tar::Archive;
use futures_util::StreamExt;
use reqwest::{Client, Response};
use serde::Deserialize;
use tokio::io::BufReader;
use tokio_util::{compat::TokioAsyncReadCompatExt, io::StreamReader};

pub const SITE_LI: &[&str] = &[
  "registry.npmjs.org",
  "registry.npmmirror.com",
  "mirrors.cloud.tencent.com/npm",
  "mirrors.huaweicloud.com/repository/npm",
];

#[derive(Deserialize, Debug)]
pub struct Info {
  version: String,
}

pub struct Pkg {
  pub name: String,
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

impl Pkg {
  pub fn new(name: impl Into<String>) -> Self {
    Self { name: name.into() }
  }

  pub async fn latest(&self) -> Result<String> {
    let bin = response(&self.name, "latest").await?.bytes().await?;
    let info: Info = sonic_rs::from_slice(&bin)?;
    Ok(info.version)
  }

  pub async fn tgz(&self, ver: impl AsRef<str>, out: impl AsRef<Path>) -> Null {
    let out = out.as_ref();
    let stream = response(
      &self.name,
      &format!(
        "-/{}-{}.tgz",
        self.name.split("/").last().unwrap_or(""),
        ver.as_ref()
      ),
    )
    .await?
    .bytes_stream();

    let reader = StreamReader::new(
      stream
        .map(|result| result.map_err(|e| tokio::io::Error::new(tokio::io::ErrorKind::Other, e))),
    );

    let gz_decoder = GzipDecoder::new(BufReader::new(reader)).compat();

    if let Ok(meta) = tokio::fs::metadata(out).await {
      if meta.is_dir() {
        tokio::fs::remove_dir_all(out).await?;
      } else {
        tokio::fs::remove_file(out).await?;
      }
    }

    let archive = Archive::new(gz_decoder);
    let mut entries = archive.entries()?;
    while let Some(entry) = entries.next().await {
      if let Ok(mut f) = xerr::ok!(entry) {
        if let Ok(path) = xerr::ok!(f.path()) {
          let path = path.display().to_string();
          if let Some(path) = path.strip_prefix("package") {
            let path = &path[1..];
            let fp = out.join(path);

            if let Err(err) = f.unpack(&fp).await {
              let kind = err.kind();
              if kind == tokio::io::ErrorKind::NotFound {
                if let Some(parent) = fp.parent() {
                  tokio::fs::create_dir_all(parent).await?;
                  f.unpack(&fp).await?;
                }
              } else {
                tracing::error!("{}: {}", kind, err);
              }
            }
          }
        }
      }
    }

    OK
  }
}
