use std::time::Duration;

use crate::{
  error::{Error, Result},
  route::{Protocol, Upstream},
};

/// 代理请求到上游服务器
pub async fn proxy(req: reqwest::Request, upstream: &Upstream) -> Result<reqwest::Response> {
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

  // 修改请求 URI
  let mut path_and_query = req.url().path().to_string();
  if let Some(query) = req.url().query() {
    path_and_query.push('?');
    path_and_query.push_str(query);
  }

  let url = format!(
    "{}://{}{}",
    if upstream.protocol == Protocol::H1 {
      "http"
    } else {
      "https"
    },
    upstream_addr,
    path_and_query
  );

  // 复制请求方法、头和体
  let method = req.method().clone();
  let headers = req.headers().clone();
  let body = if let Some(body) = req.body() {
    body.as_bytes().unwrap().to_vec()
  } else {
    Vec::new()
  };

  // 发送请求
  let res = client
    .request(method, &url)
    .headers(headers)
    .body(body)
    .send()
    .await?;

  Ok(res)
}
