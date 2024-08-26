use std::path::Path;

use aok::{Null, Result, OK};
use aws_sdk_s3::{
  config::{Credentials, Region},
  Client as S3Client, Config,
};
use futures::{stream::FuturesUnordered, StreamExt};
use gxhash::HashMap;
use i18::env::I18N_SITE_YML_PATH;
use serde::{Deserialize, Serialize};

use super::Seo;

pub const MIME_TYPE_GZIP: &str = "application/gzip";
pub const MIME_TYPE_HTML: &str = "text/html";
pub const MIME_TYPE_XML: &str = "text/xml";

fn mime_type(rel: &str) -> Option<&'static str> {
  match rel.rsplit('.').next() {
    Some("gz") => Some(MIME_TYPE_GZIP),
    Some("htm") | Some("html") => Some(MIME_TYPE_HTML),
    Some("xml") => Some(MIME_TYPE_XML),
    _ => None,
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
  pub endpoint: String,
  pub region: Option<String>,
  pub ak: String,
  pub sk: String,
  pub bucket: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct I18nConf {
  pub s3: Option<HashMap<String, Vec<Conf>>>,
}

#[derive(Debug)]
pub struct Client {
  pub s3: S3Client,
  pub bucket: String,
}

pub struct S3 {
  li: Vec<Client>,
}

impl Seo for S3 {
  async fn put(&self, rel: impl AsRef<str>, bin: impl AsRef<[u8]>) -> Null {
    if self.li.is_empty() {
      return OK;
    }
    let rel = rel.as_ref();
    let bin = bin.as_ref();

    let mut futures = FuturesUnordered::new();
    let mime_type = mime_type(rel);

    {
      for client in &self.li {
        let bin = bin.to_vec();
        futures.push(async move {
          let mut put_request = client
            .s3
            .put_object()
            .bucket(&client.bucket)
            .key(rel)
            .body(bin.into());
          if let Some(mime_type) = mime_type {
            put_request = put_request.content_type(mime_type);
          }
          put_request.send().await
        });
      }
      while let Some(r) = futures.next().await {
        r?;
      }
    }
    OK
  }

  fn init(_root: &Path, name: &str, host: &str) -> Result<Self> {
    let conf: I18nConf = i18::env::load()?;
    let mut li = Vec::new();

    if let Some(mut s3) = conf.s3
      && let Some(confs) = s3.remove(host)
    {
      for conf in confs {
        let config = Config::builder()
          .credentials_provider(Credentials::new(&conf.ak, &conf.sk, None, None, ""))
          .endpoint_url(if conf.endpoint.contains("://") {
            conf.endpoint.into()
          } else {
            format!("https://{}", conf.endpoint)
          })
          .region(if let Some(region) = conf.region {
            Region::new(region)
          } else {
            // 必须要设置一个
            Region::new("0")
          });

        let s3 = S3Client::from_conf(config.build());

        li.push(Client {
          s3,
          bucket: conf.bucket,
        });
      }
    } else {
      eprintln!("❌ {} : s3 {name} not found", I18N_SITE_YML_PATH.display());
    }

    Ok(Self { li })
  }
}
