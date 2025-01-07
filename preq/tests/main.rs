genv::s!(IPV6_PROXY_TEST_URL, IPV6_PROXY_TEST_RESULT);
use aok::{Result, OK};

#[tokio::test]
async fn test() -> Result<()> {
  loginit::init();
  use preq::PROXY;
  let url = IPV6_PROXY_TEST_URL.as_str();
  // let url = "https://www.baidu.com";
  let r = PROXY.post_form(url, [("q", "I")]).await?;
  tracing::info!("{}", String::from_utf8_lossy(&r));
  // assert_eq!(r, &*IPV6_PROXY_TEST_RESULT);
  OK
}
