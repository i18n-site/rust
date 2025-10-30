use std::{
  cmp::max,
  fmt,
  sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
  },
  time::{Duration, Instant},
};

use reqwest::{header::HeaderMap, Body, IntoUrl, Method, StatusCode};
use reqwest_client::CLIENT;
use tokio::{task::JoinHandle, time::sleep};
use zset::{Api, Zset};

use crate::{proxy::Proxy, Response, Result};

pub struct Fetch {
  pub proxy_zset: Arc<Zset<String, Proxy, i64>>,
  pub cron: Option<JoinHandle<()>>,
}

impl Drop for Fetch {
  fn drop(&mut self) {
    if let Some(handle) = self.cron.take() {
      handle.abort();
    }
  }
}
impl fmt::Debug for Fetch {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Fetch")
      .field("proxy_zset_len", &self.proxy_zset.len())
      .finish()
  }
}

pub fn score_err(score: i64) -> i64 {
  if score < 0 {
    score / 2
  } else {
    score + 600
  }
}

pub static TOTAL_COST: AtomicU64 = AtomicU64::new(0);
pub static TOTAL_REQ: AtomicU64 = AtomicU64::new(0);

impl Fetch {
  pub fn next(&self) -> Option<(Arc<Proxy>, i64)> {
    let len = self.proxy_zset.len();
    if len == 0 {
      return None;
    }
    let pos = biased::rng(0..max(1, len / 2), 1.8);
    self.proxy_zset.get_with_score(pos)
  }

  pub async fn rand(&self) -> Option<(Arc<Proxy>, i64)> {
    for _ in 0..3 {
      if let Some(proxy) = self.next() {
        return Some(proxy);
      } else {
        sleep(Duration::from_secs(6)).await;
      }
    }
    None
  }

  pub async fn run<B: Into<Body>>(
    &self,
    method: Method,
    url: impl IntoUrl,
    headers: HeaderMap,
    body: Option<B>,
  ) -> Result<Response> {
    macro_rules! proxy {
      ($client:expr) => {{
        let client = $client;
        let mut request_builder = client.request(method, url);
        request_builder = request_builder.headers(headers);
        if let Some(body) = body {
          request_builder = request_builder.body(body.into());
        }
        async move {
          let response = request_builder.send().await?;
          let status = response.status();
          let headers = response.headers().clone();
          let body = response.bytes().await?;
          Ok(Response {
            status,
            headers,
            body,
          })
        }
        .await
      }};
    }

    if let Some((proxy, mut score)) = self.rand().await {
      let start = Instant::now();
      match proxy!(&proxy) {
        Err(err) => {
          score = score_err(score);
          eprintln!("{} score {} {}", &proxy.name, -score, err);
          self.proxy_zset.add(proxy, score);
          Err(err)
        }
        Ok(response) => {
          let status = response.status;

          'out: {
            if matches!(status, StatusCode::OK) {
              let cost = start.elapsed().as_secs();
              println!("{status} score {} cost {cost}s {}", -score, &proxy.name);
              if score > 0 {
                score /= 2;
              } else {
                let total_cost = TOTAL_COST.fetch_add(cost, Ordering::Relaxed);
                let total_req = TOTAL_REQ.fetch_add(1, Ordering::Relaxed) + 1;
                if total_req > u64::MAX / 2 {
                  TOTAL_COST.store(total_cost / 2, Ordering::Relaxed);
                  TOTAL_REQ.store(total_req / 2, Ordering::Relaxed);
                }
                let avg_cost = (total_cost / total_req) as i64;
                let cost = cost as i64;
                let 超时 = cost - 1 - avg_cost;
                // 加分就是降权
                score += 超时;
              }
            } else if matches!(response.status, StatusCode::NOT_FOUND) {
              break 'out;
            } else {
              score = score_err(score);
            };
            self.proxy_zset.add(proxy, score);
          }
          Ok(response)
        }
      }
    } else {
      proxy!(&CLIENT)
    }
  }
}
