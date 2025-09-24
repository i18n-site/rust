use std::{sync::Arc, time::Duration};

use reqwest::{IntoUrl, Url};
use tokio::time::sleep;
use zset::Zset;

use crate::{refresh_li, Fetch, Proxy, Result};

pub async fn load<S: IntoUrl>(proxy_subscription_li: impl IntoIterator<Item = S>) -> Result<Fetch> {
  let proxy_zset = Arc::new(Zset::<String, Proxy, i64>::new());

  let proxy_subscription_li = proxy_subscription_li
    .into_iter()
    .map(|s| s.into_url())
    .collect::<std::result::Result<Vec<Url>, _>>()?;
  refresh_li(&proxy_subscription_li, proxy_zset.clone()).await?;

  let cron = tokio::spawn({
    let proxy_zset = proxy_zset.clone();
    async move {
      loop {
        sleep(Duration::from_hours(1)).await;
        let _ = refresh_li(&proxy_subscription_li, proxy_zset.clone()).await;
      }
    }
  });

  Ok(Fetch {
    proxy_zset,
    cron: Some(cron),
  })
}
