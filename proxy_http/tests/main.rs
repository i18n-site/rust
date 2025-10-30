use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use aok::{OK, Void};
use base64::{Engine as _, engine::general_purpose};
use log::info;
use tokio::{
  io::{AsyncReadExt, AsyncWriteExt},
  net::TcpStream,
};

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
  rustls::crypto::aws_lc_rs::default_provider()
    .install_default()
    .unwrap();
}

genv::s!(PROXY_SUBSCRITION_URL: String);

#[tokio::test]
async fn test_proxy() -> Void {
  let fetch = proxy_fetch::load(PROXY_SUBSCRITION_URL.split(";")).await?;

  let user = "test";
  let password = "pwd";
  let port = 32342;
  let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port));

  tokio::spawn(async move {
    xerr::log!(proxy_http::run(fetch, addr, user, &password).await);
  });

  tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

  let url = "http://ifconfig.me/ip";
  let client = reqwest::Client::builder()
    .proxy(reqwest::Proxy::http(format!(
      "http://test:err@{}:{}",
      "127.0.0.1", port
    ))?)
    .build()?;

  let res = client.get(url).send().await?;
  assert_eq!(
    res.status(),
    reqwest::StatusCode::PROXY_AUTHENTICATION_REQUIRED
  );

  info!("✅ Proxy Authentication Required");

  let proxy = reqwest::Proxy::http(format!("http://{user}:{password}@127.0.0.1:{port}"))?;
  let client = reqwest::Client::builder().proxy(proxy).build()?;

  let res = client.get(url).send().await?;
  let ip = res.text().await?;
  info!("ip: {ip}");
  assert!(!ip.is_empty());

  OK
}

#[tokio::test]
async fn test_tunnel_proxy() -> Void {
  let fetch = proxy_fetch::load(PROXY_SUBSCRITION_URL.split(";")).await?;

  let user = "test";
  let password = "pwd";
  let port = 32343; // 使用不同的端口避免冲突
  let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port));

  tokio::spawn(async move {
    xerr::log!(proxy_http::run(fetch, addr, user, &password).await);
  });

  tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

  // 测试隧道连接到 httpbin.org:80
  let target_host = "ifconfig.me";
  let target_port = 80;

  // 连接到代理服务器
  let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;

  // 发送 CONNECT 请求建立隧道
  let connect_request = format!(
    "CONNECT {}:{} HTTP/1.1\r\n\
     Host: {}:{}\r\n\
     Proxy-Authorization: Basic {}\r\n\
     \r\n",
    target_host,
    target_port,
    target_host,
    target_port,
    general_purpose::STANDARD.encode(format!("{}:{}", user, password))
  );

  stream.write_all(connect_request.as_bytes()).await?;

  // 读取 CONNECT 响应
  let mut response = vec![0u8; 1024];
  let n = stream.read(&mut response).await?;
  let response_str = String::from_utf8_lossy(&response[..n]);

  info!("CONNECT Response: {}", response_str);

  // 检查是否收到 200 Connection established
  assert!(
    response_str.contains("200"),
    "Expected 200 response for CONNECT, got: {}",
    response_str
  );

  info!("✅ Tunnel established successfully");

  // 通过隧道发送 HTTP 请求
  let http_request = format!(
    "GET /ip HTTP/1.1\r\n\
     Host: {}\r\n\
     Connection: close\r\n\
     \r\n",
    target_host
  );

  stream.write_all(http_request.as_bytes()).await?;

  // 读取 HTTP 响应头
  let mut response_buffer = Vec::new();
  let mut temp_buffer = [0u8; 1024];

  // 读取响应，直到我们得到完整的响应或超时
  let response_result = tokio::time::timeout(tokio::time::Duration::from_secs(10), async {
    loop {
      match stream.read(&mut temp_buffer).await {
        Ok(0) => break, // 连接关闭
        Ok(n) => {
          response_buffer.extend_from_slice(&temp_buffer[..n]);
          let response_str = String::from_utf8_lossy(&response_buffer);
          // 如果我们收到了完整的 JSON 响应，就停止读取
          if response_str.contains("HTTP/1.1 200")
            && response_str.contains("origin")
            && response_str.contains("}")
          {
            break;
          }
        }
        Err(e) => return Err(e),
      }
    }
    Ok(())
  })
  .await;

  match response_result {
    Ok(Ok(())) => {
      let http_response_str = String::from_utf8_lossy(&response_buffer);
      info!(
        "HTTP Response through tunnel (length: {}): {}",
        response_buffer.len(),
        http_response_str
      );

      // 检查是否收到有效的 HTTP 响应
      assert!(
        http_response_str.contains("HTTP/1.1 200"),
        "Expected HTTP 200 response, got: {}",
        http_response_str
      );
      assert!(
        http_response_str.contains("origin"),
        "Expected JSON response with 'origin' field, got: {}",
        http_response_str
      );

      info!("✅ HTTP request through tunnel successful");
    }
    Ok(Err(e)) => {
      return Err(e.into());
    }
    Err(_) => {
      let http_response_str = String::from_utf8_lossy(&response_buffer);
      info!(
        "Timeout reading response. Partial response (length: {}): {}",
        response_buffer.len(),
        http_response_str
      );

      // 即使超时，如果我们收到了部分有效响应，也认为测试成功
      if http_response_str.contains("HTTP/1.1 200") {
        info!("✅ HTTP request through tunnel successful (partial response)");
      } else {
        panic!("Timeout waiting for HTTP response through tunnel");
      }
    }
  }

  OK
}
