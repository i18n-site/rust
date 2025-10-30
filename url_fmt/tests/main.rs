use anyhow::Result;
use log::info;
use url_fmt::url_fmt;

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
}

// #[tokio::test]
// async fn test_async() -> Result<()>{ {
//   info!("async {}", 123456);
//   Ok(())
// }

#[test]
fn test() -> Result<()> {
  info!("> test url_fmt");

  let tests = [
    (
      "trojan://YWVzLTI1Ni1nY206cGFzc3dvcmQ@example.com:8388#Node1",
      "trojan://*@example.com:8388#Node1",
    ),
    (
      "vless://YWVzLTI1Ni1nY206cGFzc3dvcmQ@example.com:8388#%E8%8A%82%E7%82%B91",
      "vless://*@example.com:8388#节点1",
    ),
    (
      "ss://YWVzLTI1Ni1nY206cGFzc3dvcmQ@example.com:8388",
      "ss://*@example.com:8388",
    ),
    (
      "ss://user:pass@word@example.com:8388#Node",
      "ss://*@example.com:8388#Node",
    ),
    ("http://example.com", "http://example.com"),
    ("ss://example.com:8388#Node", "ss://example.com:8388#Node"),
    (
      "http://user:password@example.com/path?query=1#fragment",
      "http://*@example.com/path?query=1#fragment",
    ),
    ("custom://user@host:port/path", "custom://*@host:port/path"),
    (
      "custom://user@host:port/path#%E4%B8%AD%E6%96%87",
      "custom://*@host:port/path#中文",
    ),
  ];

  for (url, expected) in tests {
    let formatted_url = url_fmt(url);
    assert_eq!(formatted_url, expected);
    info!(
      "
{}
->
{}
",
      url, formatted_url
    );
  }

  info!("");

  Ok(())
}
