use std::{path::Path, time::Duration};

use aok::{Null, Result, OK};
use aws_sdk_s3::{
  config::{timeout::TimeoutConfig, Credentials, Region},
  primitives::ByteStream,
  Client as S3Client, Config,
};
use futures::{stream::FuturesUnordered, StreamExt};

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

pub const MIME_TYPE_GZIP: &str = "application/gzip";
pub const MIME_TYPE_HTML: &str = "text/html";
pub const MIME_TYPE_XML: &str = "text/xml";
pub const MIME_TYPE_RSS: &str = "application/rss+xml";

fn mime_type(rel: &str) -> Option<&'static str> {
  match rel.rsplit('.').next() {
    Some("gz") => Some(MIME_TYPE_GZIP),
    Some("htm") | Some("html") => Some(MIME_TYPE_HTML),
    Some("xml") => Some(MIME_TYPE_XML),
    Some("rss") => Some(MIME_TYPE_RSS),
    _ => None,
  }
}

#[derive(Debug)]
pub struct Client {
  pub s3: S3Client,
  pub bucket: String,
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
      let rel = rel.to_owned();
      futures.push(async move {
        let r = async {
          let mut put_request = client
            .s3
            .put_object()
            .bucket(&client.bucket)
            .key(&rel)
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
  async fn put_path(&self, rel: impl AsRef<str> + Send, path: &str) -> Null {
    put!(self, rel, ByteStream::from_path(path).await?)
  }

  async fn put(&self, rel: impl AsRef<str>, bin: impl AsRef<[u8]>) -> Null {
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
        });
      }
    } else {
      eprintln!("❌ {} : s3 {} not found", conf_fp.display(), host);
    }

    Ok(Self { li })
  }
}
