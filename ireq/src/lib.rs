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
pub static REQ: Client = Client::builder()
  .timeout(Duration::from_secs(120))
  .connect_timeout(Duration::from_secs(16))
  .danger_accept_invalid_certs(true)
  .build()
  .unwrap();

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
