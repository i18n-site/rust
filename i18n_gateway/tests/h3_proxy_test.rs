mod i18n_gateway_srv;
mod util;
use std::net::SocketAddr;

use i18n_gateway_srv::{TEST_HOST, TEST_RESPONSE_BODY};
use reqwest::ClientBuilder;
use tokio::time::{Duration, sleep};

#[tokio::test]
async fn test_h3_proxy() -> anyhow::Result<()> {
  // sleep 1s to wait for the server to start
  sleep(Duration::from_secs(1)).await;

  let path = "/test-path";
  let h3_addr: SocketAddr = i18n_gateway_srv::H3_ADDR.parse()?;

  let builder = |c: ClientBuilder| c.http3_prior_knowledge();

  let res = util::get_with_builder(&format!("https://{TEST_HOST}{path}"), h3_addr, builder).await?;

  let body = res.text().await?;
  println!("h3 body: {}", body);
  assert_eq!(body, String::from_utf8_lossy(TEST_RESPONSE_BODY));

  println!("h3 proxy test passed");

  Ok(())
}
