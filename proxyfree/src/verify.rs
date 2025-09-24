// src/verify.rs

use std::time::Duration;

use reqwest::StatusCode;
use tokio::time::timeout;

use crate::proxy::Proxy;

const VERIFY_URL: &str = "https://www.taobao.com/robots.txt";
const VERIFY_CONTENT: &str = "Allow:";
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const VERIFY_TIMEOUT: Duration = Duration::from_secs(30);

pub async fn verify(proxy: &Proxy) -> bool {
  timeout(VERIFY_TIMEOUT, async {
    let client = match reqwest::Client::builder()
      .proxy(reqwest::Proxy::all(proxy.to_string()).unwrap())
      .connect_timeout(CONNECT_TIMEOUT)
      .build()
    {
      Ok(c) => c,
      Err(_) => return false,
    };

    let response = match client.get(VERIFY_URL).send().await {
      Ok(res) => res,
      Err(_) => return false,
    };

    if response.status() != StatusCode::OK {
      return false;
    }

    match response.text().await {
      Ok(text) => text.contains(VERIFY_CONTENT),
      Err(_) => false,
    }
  })
  .await
  .unwrap_or_default()
}
