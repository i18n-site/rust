use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_ss_proxy::{SsConnector, SsMiddleware};

use crate::error::Result;

pub enum ProxyType {
  Ss,
}

pub struct Proxy {
  pub proxy_type: ProxyType,
  pub client: ClientWithMiddleware,
}

impl Proxy {
  pub fn ss_new(url: &str) -> Result<Self> {
    // 1. 创建 SsConnector
    let connector = SsConnector::new(url)?;

    // 2. 从连接器创建 SsMiddleware
    let ss_middleware = SsMiddleware::new(connector);

    // 3. 使用中间件构建 reqwest 客户端
    let client = reqwest::Client::builder().no_proxy().build()?;
    let client: ClientWithMiddleware = ClientBuilder::new(client).with(ss_middleware).build();
    Ok(Self {
      proxy_type: ProxyType::Ss,
      client,
    })
  }
}
