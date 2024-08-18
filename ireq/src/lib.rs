use std::time::Duration;

use aok::Result;
use bytes::Bytes;
use reqwest::{Body, Client, IntoUrl, RequestBuilder, StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReqError {
  #[error("{0} {1}")]
  Status(StatusCode, String),
}

#[static_init::dynamic]
pub static REQ: Client = {
  let b = Client::builder()
    .timeout(Duration::from_secs(100))
    .zstd(true)
    .connect_timeout(Duration::from_secs(9));

  #[cfg(feature = "proxy")]
  let b = {
    use reqwest::Proxy;
    // export https_proxy=http://127.0.0.1:7890 http_proxy=http://127.0.0.1:7890 all_proxy=socks5://127.0.0.1:7890
    let mut b = b;
    #[allow(clippy::never_loop)]
    'out: loop {
      for i in ["all", "http"] {
        if let Ok(url) = std::env::var(format!("{i}_proxy")) {
          b = b.proxy(Proxy::http(url).unwrap());
          break 'out;
        }
      }
      if let Ok(url) = std::env::var("https_proxy") {
        b = b.proxy(Proxy::https(url).unwrap());
      }

      break;
    }
    b
  };

  b.build().unwrap()
};

pub async fn req(req: RequestBuilder) -> Result<Bytes> {
  let res = req.send().await?;
  let status = res.status();
  let bin = res.bytes().await?;
  if status != StatusCode::OK {
    Err(ReqError::Status(status, String::from_utf8_lossy(&bin).into()).into())
  } else {
    Ok(bin)
  }
}

pub async fn getbin(url: impl IntoUrl) -> Result<Bytes> {
  let url = url.into_url()?;
  req(REQ.get(url)).await
}

pub async fn get(url: impl IntoUrl) -> Result<String> {
  let url = url.into_url()?;
  Ok(String::from_utf8_lossy(&req(REQ.get(url)).await?).into())
}

macro_rules! method {
  ($($method: ident),*) => {
    $(
    pub async fn $method(url: impl IntoUrl, body:impl Into<Body>) -> Result<String> {
      let url = url.into_url()?;
      let r = REQ.$method(url.clone()).body(body);
      Ok(String::from_utf8_lossy(&req(r).await?).into())
    }
    )*
  };
}

method!(post, delete, patch, put);
