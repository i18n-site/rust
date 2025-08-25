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
  // 选择一个上游服务器地址（简单轮询）
  let upstream_addr = upstream.addr_li.first().ok_or(Error::UpstreamNotFound)?;

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

  let url = format!(
    "{}://{upstream_addr}{path_and_query}",
    if upstream.protocol == Protocol::H1 {
      "http"
    } else {
      "https"
    },
  );

  // 发送请求
  let res = client
    .request(method, &url)
    .headers(headers)
    .body(body)
    .send()
    .await?;

  Ok(res)
}
