use std::{path::Path, time::Duration};

use aok::{OK, Result, Void};
use aws_sdk_s3::{
  Client as S3Client, Config,
  config::{Credentials, Region, timeout::TimeoutConfig},
  primitives::ByteStream,
};
use futures::{StreamExt, stream::FuturesUnordered};

pub mod yml {
  use gxhash::HashMap;
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct Ckv {
    pub baidu: Option<String>,
    pub indexnow: Option<String>,
    pub google: Option<String>,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct Conf {
    pub endpoint: String,
    pub region: Option<String>,
    pub ak: String,
    pub sk: String,
    pub bucket: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub prefix: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct Site {
    pub s3: Option<Vec<Conf>>,
    pub seo: Ckv,
  }

  #[derive(Debug, Serialize, Deserialize, Default)]
  pub struct I18nConf {
    pub site: Option<HashMap<String, Site>>,
  }
}

use super::Ckv;

pub mod mime {
  pub const GZIP: &str = "application/gzip";
  pub const HTML: &str = "text/html";
  pub const XML: &str = "text/xml";
  pub const CSS: &str = "text/css";
  pub const JS: &str = "application/javascript";
  pub const TXT: &str = "text/plain";
  pub const RSS: &str = "application/rss+xml";
  pub const ICO: &str = "image/x-icon";
  pub const AVIF: &str = "image/avif";
  pub const PNG: &str = "image/png";
}

fn mime_type(rel: &str) -> Option<&'static str> {
  if let Some(ext) = rel.rsplit('.').next() {
    Some(match ext {
      "avif" => mime::AVIF,
      "css" => mime::CSS,
      "gz" => mime::GZIP,
      "htm" | "html" => mime::HTML,
      "ico" => mime::ICO,
      "js" | "mjs" => mime::JS,
      "png" => mime::PNG,
      "rss" => mime::RSS,
      "txt" | "md" => mime::TXT,
      "xml" => mime::XML,
      _ => return None,
    })
  } else {
    None
  }
}

#[derive(Debug)]
pub struct Client {
  pub s3: S3Client,
  pub bucket: String,
  pub prefix: String,
}

pub struct S3 {
  li: Vec<Client>,
}

macro_rules! put {
  ($self:expr,$rel:expr, $body:expr) => {{
    let li = &$self.li;
    if li.is_empty() {
      return OK;
    }
    let rel = $rel.as_ref();
    let mime_type = mime_type(rel);

    let mut futures = FuturesUnordered::new();

    for client in li {
      futures.push(async move {
        let r = async {
          let mut put_request = client
            .s3
            .put_object()
            .bucket(&client.bucket)
            .key(format!("{}{}", client.prefix, rel))
            .body($body);
          if let Some(mime_type) = mime_type {
            put_request = put_request.content_type(mime_type);
          }
          put_request.send().await?;
          Ok::<_, aok::Error>(())
        }
        .await;
        (client, r)
      });
    }

    while let Some((client, r)) = futures.next().await {
      if let Err(e) = r {
        let conf = client.s3.config();
        eprintln!("{:?}", conf);
        return Err(e.into());
      }
    }

    OK
  }};
}

impl Ckv for S3 {
  async fn put_path(&self, rel: impl AsRef<str> + Send, path: &str) -> Void {
    put!(self, rel, ByteStream::from_path(path).await?)
  }

  async fn put(&self, rel: impl AsRef<str>, bin: impl AsRef<[u8]>) -> Void {
    let bin = bin.as_ref();
    put!(self, rel, bin.to_vec().into())
  }
}

impl S3 {
  pub fn load(conf_fp: &Path, host: &str) -> Result<Self> {
    let conf: yml::I18nConf = serde_yaml::from_slice(&ifs::r(conf_fp)?)?;
    let mut li = Vec::new();

    if let Some(mut site) = conf.site
      && let Some(site) = site.remove(host)
      && let Some(confs) = site.s3
    {
      for conf in confs {
        let config = Config::builder()
          .credentials_provider(Credentials::new(&conf.ak, &conf.sk, None, None, ""))
          .retry_config(aws_sdk_s3::config::retry::RetryConfig::adaptive().with_max_attempts(16))
          .endpoint_url(if conf.endpoint.contains("://") {
            conf.endpoint
          } else {
            format!("https://{}", conf.endpoint)
          })
          .region(if let Some(region) = conf.region {
            Region::new(region)
          } else {
            // 必须要设置一个
            Region::new("x")
          })
          .timeout_config(
            TimeoutConfig::builder()
              .connect_timeout(Duration::from_secs(10))
              .read_timeout(Duration::from_secs(100))
              .build(),
          );

        let s3 = S3Client::from_conf(config.build());

        li.push(Client {
          s3,
          bucket: conf.bucket,
          prefix: conf.prefix,
        });
      }
    } else {
      eprintln!("❌ {} : s3 {} not found", conf_fp.display(), host);
    }

    Ok(Self { li })
  }
}
