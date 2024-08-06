genv::s!(IPV6_PROXY_TEST_URL, IPV6_PROXY_TEST_RESULT);

#[tokio::test]
async fn test() -> aok::Result<()> {
  loginit::init();
  let url = &*IPV6_PROXY_TEST_URL;
  tracing::info!("{}", url);

  use preq1::{IPV6_PROXY, IPV6_PROXY_PORT};
  let proxy_li: String = IPV6_PROXY();
  for proxy in proxy_li.split(' ') {
    let port: String = IPV6_PROXY_PORT();
    let proxy = format!("{}:{}", proxy, port);
    tracing::info!("proxy {}", proxy);
    let proxy = preq1::proxy(proxy.as_str());
    // let url = "https://www.baidu.com";
    let r = preq1::post_form(0, &[proxy], url, [("q", "I")]).await?;
    tracing::info!("{}", String::from_utf8_lossy(&r));
    assert_eq!(r, &*IPV6_PROXY_TEST_RESULT);
  }
  aok::OK
}
