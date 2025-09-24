use std::sync::Arc;

use hyper::{Request, body::Incoming, service::service_fn, upgrade::Upgraded};
use log::error;
use proxy_fetch::Fetch;

use crate::proxy;

pub async fn upgrade(host: String, port: u16, upgraded: Upgraded, fetch: Arc<Fetch>) {
  let base = if port == 443 || port == 80 {
    format!("https://{host}")
  } else {
    format!("https://{host}:{port}")
  };

  let service = service_fn(move |req: Request<Incoming>| {
    let fetch = Arc::clone(&fetch);
    let base = base.clone();

    async move {
      let url = format!(
        "{}{}",
        base,
        req
          .uri()
          .path_and_query()
          .map(|i| i.as_str())
          .unwrap_or_default()
      );

      proxy(url, req, fetch).await
    }
  });

  // 直接使用 Upgraded，它已经实现了 hyper::rt::Read 和 hyper::rt::Write
  // Use Upgraded directly, it already implements hyper::rt::Read and hyper::rt::Write
  if let Err(err) =
    hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new())
      .serve_connection(upgraded, service)
      .await
  {
    error!("Error serving tunneled connection: {:?}", err);
  }
}
