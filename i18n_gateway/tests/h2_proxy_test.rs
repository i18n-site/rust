mod i18n_gateway_srv;
mod util;
use std::net::SocketAddr;

use i18n_gateway_srv::{TEST_HOST, TEST_RESPONSE_BODY};
use tokio::time::{Duration, sleep};

#[tokio::test]
async fn test_h2_proxy() -> anyhow::Result<()> {
  // sleep 1s to wait for the server to start
  sleep(Duration::from_secs(1)).await;
  let path = "/test-path";
  let h2_addr: SocketAddr = i18n_gateway_srv::H2_ADDR.parse()?;

  let body = util::get_body(&format!("https://{TEST_HOST}{path}"), h2_addr).await?;

  println!("h2 body: {}", body);
  assert_eq!(body, String::from_utf8_lossy(TEST_RESPONSE_BODY));

  println!("h2 proxy test passed");

  Ok(())
}
