use std::time::Duration;

use bytes::Bytes;
use http::{HeaderMap, method::Method};

use crate::{
  error::{Error, Result},
  route::{Protocol, Upstream},
};

static mut N: usize = 0;

/// 代理请求到上游服务器
pub async fn proxy(
  method: Method,
  path_and_query: &str,
  headers: HeaderMap,
  body: Option<Bytes>,
  upstream: &Upstream,
) -> Result<reqwest::Response> {
  if upstream.addr_li.is_empty() {
    return Err(Error::UpstreamNotFound);
  }

  // 构建 reqwest 客户端
  let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(upstream.request_timeout_sec))
    .connect_timeout(Duration::from_secs(upstream.connect_timeout_sec));

  let client = match upstream.protocol {
    Protocol::H3 => client.http3_prior_knowledge(),
    Protocol::H2 => client.http2_prior_knowledge(),
    _ => client,
  };

  let client = client.build()?;

  let scheme = if upstream.protocol == Protocol::H1 {
    "http"
  } else {
    "https"
  };

  let len = upstream.addr_li.len();

  let mut pos = unsafe {
    N = N.overflowing_add(1).0;
    N
  };

  let mut retryed = 0;
  loop {
    let upstream_addr = &upstream.addr_li[pos % len];
    let url = format!("{scheme}://{upstream_addr}{path_and_query}");

    let mut req_builder = client
      .request(method.clone(), &url)
      .headers(headers.clone());
    if let Some(ref body_bytes) = body {
      req_builder = req_builder.body(body_bytes.clone());
    }

    match req_builder.send().await {
      Ok(res) => return Ok(res),
      Err(e) => {
        tracing::warn!("proxy failed: {upstream_addr} {e}");
        retryed += 1;
        if retryed < upstream.max_retry {
          pos = pos.overflowing_add(1).1;
          continue;
        }
        return Err(e.into());
      }
    }
  }
}
