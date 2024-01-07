#[tokio::test]
async fn test() -> aok::Result<()> {
  use preq::{IPV6_PROXY, IPV6_PROXY_PORT};
  let proxy: String = IPV6_PROXY();
  let port: String = IPV6_PROXY_PORT();
  let url = format!("http://{}:{}", proxy, port);
  let proxy = preq::proxy(url.as_str());
  preq::post(&proxy, "http://baidu.com", "").await;
  dbg!(proxy);
  aok::OK
}

/*
#[cfg(feature = "macro")]
mod test_macro {
}
*/
