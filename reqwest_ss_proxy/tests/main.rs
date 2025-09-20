use anyhow::Result;
use log::{error, info};
use percent_encoding::percent_decode_str;
use regex::Regex;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_ss_proxy::SsMiddleware;

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
}

fn hide_token_ss_url(url: &str) -> String {
  let re = Regex::new(r"ss://(.+)@").unwrap();
  let hidden_url = re.replace(url, "ss://*@").to_string();

  if let Some(hash_index) = hidden_url.find('#') {
    let base = &hidden_url[..hash_index];
    let fragment = &hidden_url[hash_index + 1..];
    if let Ok(decoded_fragment) = percent_decode_str(fragment).decode_utf8() {
      return format!("{}#{}", base, decoded_fragment);
    }
  }

  hidden_url
}

#[tokio::test]
async fn test_proxy_connection() -> Result<()> {
  let urls = match std::fs::read_to_string("tests/ss") {
    Ok(s) => s,
    Err(_) => {
      error!(
        "未找到 tests/ss 文件，跳过代理连接测试
Could not find the tests/ss file, skipping proxy connection test"
      );
      return Ok(());
    }
  };

  for url in urls.lines() {
    let url = url.trim();
    if url.is_empty() {
      continue;
    }

    info!("{}", hide_token_ss_url(url));

    // 1. Create the SsMiddleware from the URL.
    // 1. 从 URL 创建 SsMiddleware。
    let ss_middleware = SsMiddleware::from_url(url)?;

    // 3. Build the reqwest client with the middleware.
    let client = reqwest::Client::builder().no_proxy().build()?;
    let client: ClientWithMiddleware = ClientBuilder::new(client).with(ss_middleware).build();

    // 4. Send the request using the reqwest API.
    for test_url in ["http://ifconfig.me/ip", "https://ifconfig.me/ip"] {
      match client.get(test_url).send().await {
        Ok(res) => {
          let status = res.status();
          let ip = res.text().await?;
          info!("\t{test_url} {} {}", status, ip);
        }
        Err(e) => {
          error!("\t{test_url} ❌ {e}");
        }
      }
    }
  }

  Ok(())
}
