use std::time::Duration;

use reqwest::{Body, Client, IntoUrl, Response, Version};

#[static_init::dynamic]
pub static REQ: Client = Client::builder()
  .timeout(Duration::from_secs(60))
  .http3_prior_knowledge()
  .build()
  .unwrap();

pub async fn get(url: impl IntoUrl) -> reqwest::Result<Response> {
  REQ.get(url).version(Version::HTTP_3).send().await
}

macro_rules! method {
  ($($method: ident),*) => {
    $(
    pub async fn $method(url: impl IntoUrl, body:impl Into<Body>) -> reqwest::Result<Response> {
      REQ.$method(url).body(body).version(Version::HTTP_3).send().await
    }
    )*
  };
}

method!(post, delete, patch, put);
