use std::{
  fs::File,
  io::{BufRead, BufReader, ErrorKind},
  sync::Arc,
  time::Duration,
};

use anyhow::Result;
use futures::future::join_all;
use http_body_util::BodyExt;
use hyper::{Request, client::conn::http1};
use hyper_util::rt::TokioIo;
use hysteria2::{config::Config, connect};
use rustls_pki_types::ServerName;
use tokio::time::timeout;
use tokio_rustls::{TlsConnector, client::TlsStream};
use url_fmt::url_fmt;

// 常量定义
const TEST_TARGET: &str = "ifconfig.me:443";
const TEST_HOST: &str = "ifconfig.me";
const TEST_PATH: &str = "/ip";
const TIMEOUT_SECONDS: u64 = 10;

// 全局TLS配置，避免重复创建
#[static_init::dynamic]
static TLS_CONNECTOR: TlsConnector = {
  let mut root_cert_store = rustls::RootCertStore::empty();
  root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
  let tls_config = rustls::ClientConfig::builder()
    .with_root_certificates(root_cert_store)
    .with_no_client_auth();
  TlsConnector::from(Arc::new(tls_config))
};

/// 发送HTTP GET请求
async fn http_get(mut sender: http1::SendRequest<String>, path: &str) -> Result<String> {
  let req = Request::builder()
    .method("GET")
    .uri(path)
    .header("Host", TEST_HOST)
    .body(String::new())?;

  let res = sender.send_request(req).await?;
  let body = res.into_body();

  // 使用 BodyExt::collect 优化 body 读取
  let collected = body.collect().await?;
  let data = collected.to_bytes();

  Ok(String::from_utf8_lossy(&data).trim().to_string())
}

/// 建立TLS连接
async fn establish_tls_connection(
  stream: hysteria2::DuplexStream,
) -> Result<TlsStream<hysteria2::DuplexStream>> {
  let server_name = ServerName::try_from(TEST_HOST)?;
  Ok(TLS_CONNECTOR.connect(server_name, stream).await?)
}

/// 测试单个URL
async fn test_url(url: String) -> Result<()> {
  tracing::info!("test {}", url_fmt(&url));

  // 解析配置并连接
  let config = Config::from_url(&url)?;
  let client = connect(&config).await?;

  // 建立代理TCP连接
  let stream = client.tcp_connect(TEST_TARGET).await?;

  // TLS握手
  let tls_stream = establish_tls_connection(stream).await?;

  // HTTP连接
  let io = TokioIo::new(tls_stream);
  let (sender, conn) = http1::handshake(io).await?;

  // 在后台处理连接
  tokio::spawn(async move {
    if let Err(err) = conn.await {
      tracing::error!("Connection failed: {:?}", err);
    }
  });

  // 发送请求并获取IP
  let ip_addr = http_get(sender, TEST_PATH).await?;
  tracing::info!("✅ {}: {}", url_fmt(url), ip_addr);

  Ok(())
}

/// 初始化系统
fn initialize_system() -> Result<()> {
  rustls::crypto::aws_lc_rs::default_provider()
    .install_default()
    .map_err(|e| anyhow::anyhow!("Failed to install crypto provider: {:?}", e))?;
  tracing_subscriber::fmt::init();
  Ok(())
}

/// 读取服务器URL列表
/// Read server URL list
fn read_server_urls() -> Result<Vec<String>> {
  let root_dir = env!("CARGO_MANIFEST_DIR");
  let path = format!("{root_dir}/tests/hysteria2.url");
  match File::open(&path) {
    Ok(file) => {
      let reader = BufReader::new(file);
      reader
        .lines()
        .collect::<std::io::Result<Vec<String>>>()
        .map_err(Into::into)
    }
    Err(e) if e.kind() == ErrorKind::NotFound => {
      anyhow::bail!(
        "hysteria2.url file not found, please create it at {}. \n\
        The format should be one URL per line, e.g., hysteria2://password@example.com:4433\n\
        \n\
        hysteria2.url 文件未找到，请在 {} 创建。 \n\
        格式为每行一个 URL，例如：hysteria2://password@example.com:4433",
        path,
        path
      )
    }
    Err(e) => Err(e.into()),
  }
}

/// 执行并发测试
async fn run_concurrent_tests(urls: Vec<String>) -> Vec<(String, Result<()>)> {
  let tasks: Vec<_> = urls
    .into_iter()
    .map(|url| {
      tokio::spawn(async move {
        let result = timeout(Duration::from_secs(TIMEOUT_SECONDS), test_url(url.clone())).await;
        let final_result = match result {
          Ok(res) => res,
          Err(_) => Err(anyhow::anyhow!("Timeout")),
        };
        (url, final_result)
      })
    })
    .collect();

  join_all(tasks)
    .await
    .into_iter()
    .filter_map(|result| result.ok())
    .collect()
}

/// 打印测试结果
fn print_results(results: &[(String, Result<()>)]) {
  for (url, result) in results {
    let url = url_fmt(url);
    match result {
      Ok(_) => println!(
        "[OK] {}
",
        url
      ),
      Err(e) => println!(
        "[FAIL] {}: {}
",
        url, e
      ),
    }
  }
}

#[tokio::test]
async fn test() -> Result<()> {
  initialize_system()?;
  let urls = read_server_urls()?;
  let results = run_concurrent_tests(urls).await;
  print_results(&results);
  Ok(())
}
