use faststr::FastStr;
use http_body_util::Full;
use hyper::{Request, Response, StatusCode, body::Bytes, header::HOST};

use crate::error::{Error, Result};

/// 从请求中提取 Host 头
pub fn extract_host<T>(req: &Request<T>) -> Result<FastStr> {
  let host = req
    .headers()
    .get(HOST)
    .and_then(|h| h.to_str().ok())
    .or_else(|| req.uri().host())
    .ok_or(Error::NoHost)?;

  // 移除端口号
  let host = host.split(':').next().unwrap_or(host);

  Ok(FastStr::new(host))
}

/// 返回 404 响应
pub fn not_found_response() -> Response<Full<Bytes>> {
  let mut res = Response::new(Full::new(Bytes::from("Not Found")));
  *res.status_mut() = StatusCode::NOT_FOUND;
  res
}

pub async fn reqwest_to_hyper(res: reqwest::Response) -> Result<Response<Full<Bytes>>> {
  let status = res.status();
  let headers = res.headers().clone();
  let body = res.bytes().await?;

  let mut hyper_res = Response::new(Full::new(body));
  *hyper_res.status_mut() = status;
  *hyper_res.headers_mut() = headers;

  Ok(hyper_res)
}

pub fn hyper_to_reqwest_parts(
  parts: http::request::Parts,
) -> Result<(http::Method, String, http::HeaderMap)> {
  let method = parts.method.clone();
  let path_and_query = parts
    .uri
    .path_and_query()
    .map(|pq| pq.as_str())
    .unwrap_or("/")
    .to_string();
  let headers = parts.headers.clone();

  Ok((method, path_and_query, headers))
}
