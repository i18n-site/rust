use std::time::Duration;

use anyhow::Result;
use futures::future::join_all;
use log::{error, info};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_proxy::Proxy;
use tokio::time::timeout;
use url_fmt::url_fmt;

const TIMEOUT_SECONDS: u64 = 10;

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
  // Initialize the crypto provider for rustls
  // 为 rustls 初始化加密提供程序
  rustls::crypto::aws_lc_rs::default_provider()
    .install_default()
    .unwrap();
}

/// Read URLs from a file.
/// 从文件读取 URL。
fn read_urls_from_file(file_path: &str) -> Vec<String> {
  match std::fs::read_to_string(file_path) {
    Ok(s) => s
      .lines()
      .map(|s| s.trim().to_string())
      .filter(|s| !s.is_empty())
      .collect(),
    Err(_) => {
      error!(
        "Could not find {}, skipping related proxy connection tests",
        file_path
      );
      Vec::new()
    }
  }
}

/// Test a single proxy URL with a generic connector.
/// 使用通用连接器测试单个代理 URL。
async fn test_single_url(client: ClientWithMiddleware, test_url: &str) -> Result<()> {
  let res = client.get(test_url).send().await?;
  let status = res.status();
  let ip = res.text().await?;
  info!("{status} {ip}");
  Ok(())
}

async fn test_proxy_url(url: String) -> (String, Result<(), anyhow::Error>) {
  let timeout_result = timeout(Duration::from_secs(TIMEOUT_SECONDS), async {
    let middleware = Proxy::from_url(&url)?;
    let client = ClientBuilder::new(reqwest::Client::builder().no_proxy().build()?)
      .with(middleware)
      .build();

    test_single_url(client.clone(), "http://ifconfig.me/ip").await?;
    test_single_url(client, "https://ifconfig.me/ip").await?;

    Ok::<(), anyhow::Error>(())
  })
  .await;

  let result = match timeout_result {
    Ok(res) => res,
    Err(_) => Err(anyhow::anyhow!("Timeout")),
  };

  (url, result)
}

#[tokio::test]
#[cfg(any(feature = "shadowsocks", feature = "hysteria2"))]
async fn test_proxy_connection() -> Result<()> {
  #[cfg(feature = "shadowsocks")]
  let ss_urls = read_urls_from_file("tests/ss.url");
  #[cfg(not(feature = "shadowsocks"))]
  let ss_urls = Vec::<String>::new();

  #[cfg(feature = "hysteria2")]
  let hy2_urls = read_urls_from_file("tests/hysteria2.url");
  #[cfg(not(feature = "hysteria2"))]
  let hy2_urls = Vec::<String>::new();

  if ss_urls.is_empty() && hy2_urls.is_empty() {
    error!(
      "No proxy URLs found in tests/ss.url or tests/hysteria2.url, skipping tests. / 在 tests/ss.url 或 tests/hysteria2.url 中找不到代理 URL，跳过测试。"
    );
    return Ok(());
  }

  let all_urls = [ss_urls, hy2_urls].concat();
  let mut tasks = Vec::new();

  for url in all_urls {
    tasks.push(tokio::spawn(test_proxy_url(url)));
  }

  let results = join_all(tasks).await;

  // Print results
  // 打印结果
  for result in results {
    match result {
      Ok((url, Ok(_))) => {
        info!("✅ {}", url_fmt(&url));
      }
      Ok((url, Err(e))) => {
        error!("❌ {}: {}", url_fmt(&url), e);
      }
      Err(e) => {
        error!("Tokio spawn error: {}", e);
      }
    }
  }

  Ok(())
}
