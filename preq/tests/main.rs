#[tokio::test]
async fn test() -> aok::Result<()> {
  use preq::{IPV6_PROXY, IPV6_PROXY_PORT};
  let proxy: String = IPV6_PROXY();
  let port: String = IPV6_PROXY_PORT();
  let proxy = format!("http://{}:{}", proxy, port);
  let proxy = preq::proxy(proxy.as_str());
  let url = "https://translate.google.com/translate_a/t?client=gtx&tl=en&sl=zh";
  let r = preq::post_form(&proxy, url, [["q", "I"]]).await?;
  dbg!(proxy);
  dbg!(r.text().await?);
  aok::OK
}

/*
#[cfg(feature = "macro")]
mod test_macro {
}
*/
