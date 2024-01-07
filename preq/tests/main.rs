genv::s!(IPV6_PROXY_TEST_URL, IPV6_PROXY_TEST_RESULT);

#[tokio::test]
async fn test() -> aok::Result<()> {
  use preq::{IPV6_PROXY, IPV6_PROXY_PORT};
  let proxy: String = IPV6_PROXY();
  let port: String = IPV6_PROXY_PORT();
  let proxy = format!("http://{}:{}", proxy, port);
  let proxy = preq::proxy(proxy.as_str());
  let r = preq::post_form(&proxy, &*IPV6_PROXY_TEST_URL, [("q", "I")]).await?;
  let r = r.text().await?;
  dbg!(&r);
  assert_eq!(r, *IPV6_PROXY_TEST_RESULT);
  aok::OK
}

/*
#[cfg(feature = "macro")]
mod test_macro {
}
*/
