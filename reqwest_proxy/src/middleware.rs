use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;
use http_body_util::Full;
use hyper::Uri;
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::Client;
use reqwest::{Request, Response};
use reqwest_middleware::{Error, Middleware, Next};
use tower::Service;

use crate::{
  Conn,
  error::Error::HyperClient as HyperClientError,
  util::{HyperClient, body_to_stream},
};

/// `reqwest` macro_enums for proxying requests through a generic proxy server.
/// 用于通过通用代理服务器代理请求的 `reqwest` 中间件。
pub struct ProxyMiddleware<C> {
  client: HyperClient<C>,
}

impl<C: Conn + 'static> ProxyMiddleware<C>
where
  <C as Service<Uri>>::Future: Send,
{
  /// Creates a new `ProxyMiddleware` from a connector.
  /// 从连接器创建一个新的 `ProxyMiddleware`。
  pub fn new(connector: C) -> Self {
    let https = HttpsConnector::new_with_connector(connector);
    let client: HyperClient<C> = Client::builder(hyper_util::rt::TokioExecutor::new())
      .pool_idle_timeout(Duration::from_secs(90))
      .build(https);
    Self { client }
  }
}

#[async_trait]
impl<C: Conn + 'static> Middleware for ProxyMiddleware<C>
where
  <C as Service<Uri>>::Future: Send,
{
  async fn handle(
    &self,
    req: Request,
    _extensions: &mut http::Extensions,
    _next: Next<'_>,
  ) -> Result<Response, Error> {
    // Build hyper::Request from reqwest::Request
    // 从 reqwest::Request 构建 hyper::Request
    let mut hyper_req = hyper::Request::new(Full::<Bytes>::default());
    *hyper_req.method_mut() = req.method().clone();
    *hyper_req.uri_mut() = req
      .url()
      .as_str()
      .parse()
      .map_err(|e| Error::Middleware(anyhow::anyhow!("Invalid URI: {e}")))?;
    *hyper_req.headers_mut() = req.headers().clone();
    *hyper_req.version_mut() = req.version();

    // Handle the request body.
    // WARNING: This implementation buffers the entire request body into memory.
    // It does NOT support true streaming of request bodies due to limitations in `reqwest`'s API for middleware.
    // If a streaming body is encountered, an error is returned instead of silently sending an empty body.
    // 处理请求体。
    // 警告：此实现将整个请求体缓冲到内存中。
    // 由于 `reqwest` 中间件 API 的限制，它不支持请求体的真正流式传输。
    // 如果遇到流式请求体，将返回错误，而不是静默地发送空内容。
    let body_bytes = if let Some(body) = req.body() {
      if body.as_bytes().is_none() {
        // This is a streaming body, which we cannot handle correctly.
        // Returning an error is better than the previous buggy behavior of sending an empty body.
        // 这是一个流式主体，我们无法正确处理。
        // 返回错误比之前发送空主体的错误行为要好。
        return Err(Error::Middleware(anyhow::anyhow!(
          "ProxyMiddleware cannot handle streaming request bodies"
        )));
      }
      body.as_bytes().unwrap_or_default().to_vec()
    } else {
      Vec::new()
    };

    let (parts, _) = hyper_req.into_parts();
    let hyper_req = hyper::Request::from_parts(parts, Full::new(Bytes::from(body_bytes)));

    // Send the request and handle the response
    // 发送请求并处理响应
    let http_res = self
      .client
      .request(hyper_req)
      .await
      .map_err(|e| Error::Middleware(anyhow::anyhow!(HyperClientError(e))))?;

    // Convert hyper::Response back to reqwest::Response
    // 将 hyper::Response 转换回 reqwest::Response
    let (parts, body) = http_res.into_parts();
    let stream = body_to_stream(body);
    let res = hyper::Response::from_parts(parts, reqwest::Body::wrap_stream(stream));

    Ok(Response::from(res))
  }
}
