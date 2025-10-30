use std::{sync::Arc, time::Duration};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use futures::future;
use log::{info, warn};
use reqwest::Url;
use url_fmt::url_fmt;
use zset::{Api, Zset};

use crate::{error::Result, proxy::Proxy};

pub async fn refresh(
  subscription_url: Url,
  proxy_zset: Arc<Zset<String, Proxy, i64>>,
) -> Result<()> {
  let client = reqwest::Client::builder()
    .redirect(reqwest::redirect::Policy::limited(10))
    .danger_accept_invalid_certs(true)
    .build()?;

  let resp = client.get(subscription_url.clone()).send().await?;
  let body = resp.text().await?;

  let decoded = STANDARD.decode(body)?;
  let decoded = String::from_utf8_lossy(&decoded);
  for ss_url in decoded.lines() {
    let name = url_fmt(ss_url);
    if !proxy_zset.contains(&name) {
      if let Ok(proxy) = Proxy::new(&name, ss_url) {
        info!("+ {}", name);
        proxy_zset.add(proxy, 0);
      }
    }
  }
  Ok(())
}

pub async fn refresh_li(
  subscription_ss_li: &[Url],
  proxy_zset: Arc<Zset<String, Proxy, i64>>,
) -> Result<()> {
  loop {
    future::join_all(subscription_ss_li.iter().map(|url| {
      let proxy_zset = Arc::clone(&proxy_zset);
      async move {
        if let Err(err) = refresh(url.clone(), proxy_zset).await {
          warn!("refresh {} failed: {}", url, err);
        }
      }
    }))
    .await;

    if proxy_zset.is_empty() {
      warn!("ss subscription is empty");
      tokio::time::sleep(Duration::from_secs(1)).await;
    } else {
      break;
    }
  }
  Ok(())
}
