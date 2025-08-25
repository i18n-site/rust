use std::{net::SocketAddr, sync::Arc};

use http_body_util::Full;
use hyper::{
  Request, Response, StatusCode,
  body::{Bytes, Incoming},
  server::conn::http1,
  service::service_fn,
};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use super::util;
use crate::{error::Result, route::Route};

pub(super) async fn run(http_addr: SocketAddr, route: Arc<Route>) -> Result<()> {
  let listener = TcpListener::bind(http_addr).await?;
  loop {
    let (stream, _) = listener.accept().await?;
    let io = TokioIo::new(stream);
    let route = route.clone();
    tokio::task::spawn(async move {
      if let Err(err) = http1::Builder::new()
        .serve_connection(
          io,
          service_fn(move |req| handle_request(route.clone(), req)),
        )
        .await
      {
        eprintln!("h1 error : {:?}", err);
      }
    });
  }
}

async fn handle_request(
  route: Arc<Route>,
  req: Request<Incoming>,
) -> Result<Response<Full<Bytes>>> {
  let host = util::extract_host(&req)?;

  let mut res = Response::new(Full::new(Bytes::new()));
  *res.status_mut() = if route.host_conf.contains_key(&host) {
    let https_url = format!(
      "https://{host}{}",
      req
        .uri()
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("")
    );
    match hyper::header::HeaderValue::from_str(&https_url) {
      Ok(location) => {
        *res.status_mut() = StatusCode::MOVED_PERMANENTLY;
        res.headers_mut().insert("Location", location);
        return Ok(res);
      }
      Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  } else {
    return Ok(util::not_found_response());
  };
  Ok(res)
}
