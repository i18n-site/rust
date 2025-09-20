use std::{pin::Pin, time::Duration};

use async_trait::async_trait;
use bytes::Bytes;
use futures_util::{future::poll_fn, stream::Stream};
use http_body_util::Full;
use hyper::body::{Body, Incoming};
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::Client;
use reqwest::{Request, Response};
use reqwest_middleware::{Error, Middleware, Next};

use crate::{SsConnector, error::SsConnectorError};

// 定义 HyperClient 类型，使用 Full<Bytes> 作为请求体
// Define the HyperClient type, using Full<Bytes> as the request body
type HyperClient = Client<HttpsConnector<SsConnector>, Full<Bytes>>;

// 将 hyper 的 body 转换为字节流
// Convert hyper's body to a stream of bytes
fn body_to_stream(mut body: Incoming) -> impl Stream<Item = Result<Bytes, anyhow::Error>> {
  async_stream::try_stream! {
      while let Some(frame) = poll_fn(|cx| Pin::new(&mut body).poll_frame(cx)).await {
          let frame = frame.map_err(|e| anyhow::anyhow!(e))?;
          if let Some(chunk) = frame.data_ref() {
              yield chunk.clone();
          }
      }
  }
}

/// 用于通过 Shadowsocks 服务器代理请求的 `reqwest` 中间件。
/// `reqwest` middleware for proxying requests through a Shadowsocks server.
pub struct SsMiddleware {
  client: HyperClient,
}

impl SsMiddleware {
  /// 从 `SsConnector` 创建一个新的 `SsMiddleware`。
  /// Creates a new `SsMiddleware` from an `SsConnector`.
  pub fn new(connector: SsConnector) -> Self {
    let https = HttpsConnector::new_with_connector(connector);
    let client = Client::builder(hyper_util::rt::TokioExecutor::new())
      .pool_idle_timeout(Duration::from_secs(90))
      .build(https);
    Self { client }
  }

  /// 从 Shadowsocks URL 创建一个新的 `SsMiddleware`。
  /// Creates a new `SsMiddleware` from a Shadowsocks URL.
  pub fn from_url(url: &str) -> Result<Self, SsConnectorError> {
    let connector = SsConnector::new(url)?;
    Ok(Self::new(connector))
  }
}

#[async_trait]
impl Middleware for SsMiddleware {
  async fn handle(
    &self,
    req: Request,
    _extensions: &mut http::Extensions,
    _next: Next<'_>,
  ) -> Result<Response, Error> {
    // 从 reqwest::Request 构建 hyper::Request
    // Build hyper::Request from reqwest::Request
    let (mut parts, _) = hyper::Request::new(Full::<Bytes>::default()).into_parts();
    parts.method = req.method().clone();
    parts.uri = req.url().as_str().parse().unwrap();
    parts.headers = req.headers().clone();
    parts.version = req.version();

    // 处理请求体，这里有一个限制：它会将整个请求体读入内存。
    // Handle the request body, with a limitation: it reads the entire body into memory.
    let body = match req.body() {
      Some(body) => body.as_bytes().unwrap_or_default(),
      None => &[],
    };
    let hyper_req = hyper::Request::from_parts(parts, Full::new(Bytes::from(body.to_vec())));

    // 发送请求并处理响应
    // Send the request and handle the response
    let http_res = self
      .client
      .request(hyper_req)
      .await
      .map_err(|e| Error::Middleware(anyhow::anyhow!(SsConnectorError::HyperClient(e))))?;

    // 将 hyper::Response 转换回 reqwest::Response
    // Convert hyper::Response back to reqwest::Response
    let (parts, body) = http_res.into_parts();
    let stream = body_to_stream(body);
    let res = hyper::Response::from_parts(parts, reqwest::Body::wrap_stream(stream));

    Ok(Response::from(res))
  }
}
