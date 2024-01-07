#[test]
fn test() {
  use preq::{IPV6_PROXY, IPV6_PROXY_PORT};
  let proxy: String = IPV6_PROXY();
  let port: String = IPV6_PROXY_PORT();
  let url = format!("http://{}:{}", proxy, port);
  dbg!(url);
}

/*
#[cfg(feature = "macro")]
mod test_macro {
}
*/
