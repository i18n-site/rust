use std::time::Duration;

use aok::Result;
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
    .timeout(Duration::from_secs(300))
    .brotli(true)
    .zstd(true)
    .connect_timeout(Duration::from_secs(8));

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

pub async fn req(req: RequestBuilder) -> Result<String> {
  let res = req.send().await?;
  let status = res.status();
  let txt = res.text().await?;
  if status != StatusCode::OK {
    Err(ReqError::Status(status, txt).into())
  } else {
    Ok(txt)
  }
}

pub async fn get(url: impl IntoUrl) -> Result<String> {
  let url = url.into_url()?;
  req(REQ.get(url)).await
}

macro_rules! method {
  ($($method: ident),*) => {
    $(
    pub async fn $method(url: impl IntoUrl, body:impl Into<Body>) -> Result<String> {
      let url = url.into_url()?;
      let r = REQ.$method(url.clone()).body(body);
      req(r).await
    }
    )*
  };
}

method!(post, delete, patch, put);
