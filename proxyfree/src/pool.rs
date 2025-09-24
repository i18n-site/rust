use std::sync::Arc;

use crossbeam_skiplist::SkipMap;

use crate::{fetch::proxyscrape, proxy::Proxy};

// 失败次数
pub type Failed = u8;

pub struct Pool {
  pub map: Arc<SkipMap<Proxy, Failed>>,
}

impl Pool {
  pub async fn new() -> Self {
    let map = Arc::new(SkipMap::new());
    if let Ok(proxies) = proxyscrape().await {
      for proxy in proxies {
        map.insert(proxy, 0);
      }
    }
    Self { map }
  }
}
