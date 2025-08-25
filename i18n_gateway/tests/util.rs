use std::net::SocketAddr;

use reqwest::{ClientBuilder, Response};

pub async fn get_with_builder(
  url_str: &str,
  addr: SocketAddr,
  builder: impl FnOnce(ClientBuilder) -> ClientBuilder,
) -> anyhow::Result<Response> {
  let url = url::Url::parse(url_str)?;
  let host = url
    .host_str()
    .ok_or_else(|| anyhow::anyhow!("URL does not have a host"))?;
  let _path = url.path();

  let client_builder = builder(reqwest::Client::builder());

  let client = client_builder
    .redirect(reqwest::redirect::Policy::none())
    .resolve(host, addr)
    .no_proxy()
    .build()?;

  client.get(url_str).send().await.map_err(Into::into)
}

#[allow(dead_code)]
pub async fn get(url_str: &str, addr: SocketAddr) -> anyhow::Result<Response> {
  get_with_builder(url_str, addr, |c| c).await
}

#[allow(dead_code)]
pub async fn get_body(url_str: &str, addr: SocketAddr) -> anyhow::Result<String> {
  let res = get(url_str, addr).await?;
  res.text().await.map_err(Into::into)
}
