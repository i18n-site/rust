use std::time::Duration;

use reqwest::{Body, Client, IntoUrl, RequestBuilder, Version};

#[static_init::dynamic]
pub static REQ: Client = Client::builder()
  .timeout(Duration::from_secs(60))
  .http3_prior_knowledge()
  .danger_accept_invalid_certs(true)
  .build()
  .unwrap();

pub async fn req(req: RequestBuilder) -> reqwest::Result<String> {
  let res = req.version(Version::HTTP_3).send().await?;
  let txt = res.text().await?;
  if res.status() != StatusCode::OK {}
  Ok(txt)
}

pub async fn get(url: impl IntoUrl) -> reqwest::Result<String> {
  req(REQ.get(url)).await
}

macro_rules! method {
  ($($method: ident),*) => {
    $(
    pub async fn $method(url: impl IntoUrl, body:impl Into<Body>) -> reqwest::Result<String> {
      req(REQ.$method(url).body(body)).await
    }
    )*
  };
}

method!(post, delete, patch, put);
