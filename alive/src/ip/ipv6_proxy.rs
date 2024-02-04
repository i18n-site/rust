use std::net::IpAddr;

use aok::{Result, OK};

genv::s!(IPV6_PROXY_TEST_URL, IPV6_PROXY_TEST_RESULT, IPV6_PROXY_PORT);

pub async fn ping(ip: &IpAddr) -> Result<()> {
  let proxy = format!("{}:{}", ip, &*IPV6_PROXY_PORT);
  let url = &*IPV6_PROXY_TEST_URL;
  let r = preq1::post_form(0, &[preq1::proxy(proxy)], url, [("q", "I")]).await?;
  assert_eq!(r, &*IPV6_PROXY_TEST_RESULT);
  OK
}
