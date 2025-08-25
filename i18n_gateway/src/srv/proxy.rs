use std::time::Duration;

use bytes::Bytes;
use http::{HeaderMap, method::Method};

use crate::{
  error::{Error, Result},
  route::{Protocol, Upstream},
};

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

  let mut last_err: Option<Error> = None;

  // 根据 max_retry 策略进行重试
  // 最多尝试 `max_retry + 1` 次, 每次从地址列表中选择一个不同的地址
  for upstream_addr in upstream.addr_li.iter().take(upstream.max_retry + 1) {
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
        last_err = Some(e.into());
      }
    }
  }

  // 如果所有重试都失败了, 返回最后一个错误
  Err(last_err.unwrap_or(Error::UpstreamNotFound))
}
