use std::sync::Arc;

use http_body_util::{BodyExt, Full};
use hyper::{
  Request, Response, StatusCode,
  body::{Bytes, Incoming},
};
use log::{debug, error};
use proxy_fetch::Fetch;

pub async fn proxy(
  url: String,
  mut req: Request<Incoming>,
  fetch: Arc<Fetch>,
) -> std::result::Result<Response<Full<Bytes>>, hyper::Error> {
  let method = req.method().clone();
  let mut headers = req.headers().clone();

  for i in ["proxy-authorization", "proxy-connection"] {
    headers.remove(i);
  }

  let body = match req.body_mut().collect().await {
    Ok(body) => body.to_bytes(),
    Err(e) => {
      error!("Failed to collect request body: {}", e);
      let response = Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Full::new(Bytes::from("Bad Request")))
        .unwrap();
      return Ok(response);
    }
  };

  debug!("{url} {:?}", headers);
  match fetch.run(method, url, headers, Some(body)).await {
    Ok(res) => {
      let mut builder = Response::builder().status(res.status);
      for (key, value) in res.headers {
        if let Some(key) = key {
          builder = builder.header(key, value);
        }
      }
      Ok(builder.body(Full::new(res.body)).unwrap())
    }
    Err(e) => {
      error!("Fetch error: {}", e);
      Ok(
        Response::builder()
          .status(StatusCode::INTERNAL_SERVER_ERROR)
          .body(Full::new(Bytes::from("Internal Server Error")))
          .unwrap(),
      )
    }
  }
}
