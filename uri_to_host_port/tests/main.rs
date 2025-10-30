use anyhow::Result;
use hyper::Uri;
use log::info;
use parse::parse;

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
}

#[test]
fn test_parse() -> Result<()> {
  info!("> test_parse");

  // Test with http and default port
  // 测试 http 和默认端口
  let uri: Uri = "http://example.com".parse()?;
  let (host, port) = parse(&uri)?;
  assert_eq!(host, "example.com");
  assert_eq!(port, 80);

  // Test with https and default port
  // 测试 https 和默认端口
  let uri: Uri = "https://example.com".parse()?;
  let (host, port) = parse(&uri)?;
  assert_eq!(host, "example.com");
  assert_eq!(port, 443);

  // Test with explicit port
  // 测试显式端口
  let uri: Uri = "http://example.com:8080".parse()?;
  let (host, port) = parse(&uri)?;
  assert_eq!(host, "example.com");
  assert_eq!(port, 8080);

  // Test with https and explicit port
  // 测试 https 和显式端口
  let uri: Uri = "https://example.com:8443".parse()?;
  let (host, port) = parse(&uri)?;
  assert_eq!(host, "example.com");
  assert_eq!(port, 8443);

  // Test with IP address
  // 测试 IP 地址
  let uri: Uri = "http://127.0.0.1:3000".parse()?;
  let (host, port) = parse(&uri)?;
  assert_eq!(host, "127.0.0.1");
  assert_eq!(port, 3000);

  // Test with no host (should fail)
  // 测试没有主机（应该失败）
  let uri: Uri = "/path/only".parse()?;
  assert!(parse(&uri).is_err());

  // Test with invalid URI string for the new error type
  // 使用新的错误类型测试无效的 URI 字符串
  let invalid_uri = "this is not a valid uri";
  let result = parse(invalid_uri);
  assert!(result.is_err());
  let error = result.unwrap_err();
  assert_eq!(
    error.to_string(),
    format!("Invalid URI: \"{}\"", invalid_uri)
  );

  Ok(())
}
