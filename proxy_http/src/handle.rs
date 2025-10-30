use std::sync::Arc;

use http_body_util::Full;
use hyper::{
  Request, Response, StatusCode,
  body::{Bytes, Incoming},
};
use log::error;
use proxy_fetch::Fetch;

use crate::{is_authorized, proxy, upgrade};

pub async fn handle(
  req: Request<Incoming>,
  fetch: Arc<Fetch>,
  user: Arc<String>,
  password: Arc<String>,
) -> std::result::Result<Response<Full<Bytes>>, hyper::Error> {
  // 授权检查
  // Authorization check
  if !is_authorized(&req, &user, &password) {
    let mut res = Response::new(Full::new(Bytes::from("Proxy Authentication Required")));
    *res.status_mut() = StatusCode::PROXY_AUTHENTICATION_REQUIRED;
    return Ok(res);
  }

  if req.method() == hyper::Method::CONNECT {
    let uri = req.uri();
    if let Some(host) = uri.host() {
      let host = host.to_owned();
      let port = uri.port_u16().unwrap_or(443); // 默认使用 443 端口

      tokio::spawn(async move {
        match hyper::upgrade::on(req).await {
          Ok(upgraded) => {
            upgrade(host, port, upgraded, fetch).await;
          }
          Err(e) => error!("Upgrade error: {}", e),
        }
      });
      // 立即响应 200 Connection established，表示隧道已建立
      // Immediately respond with 200 Connection established to indicate that the tunnel has been established
      Ok(Response::new(Full::new(Bytes::new())))
    } else {
      let mut res = Response::new(Full::new(Bytes::from(
        "CONNECT request must have an authority",
      )));
      *res.status_mut() = StatusCode::BAD_REQUEST;
      Ok(res)
    }
  } else {
    // 处理普通 http 请求
    // Handle normal http requests
    let uri = req.uri().to_string();

    let uri = if let Some(remain) = uri.strip_prefix("http:") {
      "https:".to_string() + remain
    } else {
      uri
    };
    proxy(uri, req, fetch).await
  }
}
