use std::net::SocketAddr;

use futures::stream::{self, StreamExt};

use crate::{error, proxy::Proxy, verify::verify};

pub async fn proxyscrape() -> Result<Vec<Proxy>, error::Error> {
  let mut li = vec![];
  for proxy_type in ["socks5", "http"] {
    let url = format!(
      "https://api.proxyscrape.com/v4/free-proxy-list/get?request=displayproxies&protocol={proxy_type}&timeout=10000&country=all&ssl=all&anonymity=anonymous,elite&skip=0&limit=2000"
    );
    let body = reqwest::get(url).await?.text().await?;
    for addr in body.lines() {
      if let Ok(addr) = addr.parse::<SocketAddr>() {
        let proxy = match proxy_type {
          "socks5" => Proxy::Sock5(addr),
          "http" => Proxy::Http(addr),
          _ => unreachable!(),
        };
        li.push(proxy);
      }
    }
  }
  let verified_proxies = stream::iter(li)
    .map(|proxy| async move {
      if verify(&proxy).await {
        log::info!("✅ {proxy}");
        Some(proxy)
      } else {
        None
      }
    })
    .buffer_unordered(60)
    .filter_map(|p| async { p })
    .collect::<Vec<Proxy>>()
    .await;

  Ok(verified_proxies)
}
