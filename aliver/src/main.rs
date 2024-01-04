use std::net::SocketAddr;

use aok::Result;
use axum::{
  body::Body,
  extract::Request,
  http::{self, HeaderValue},
  middleware,
  middleware::Next,
  response::IntoResponse,
  routing::get,
  Router,
};
use tower::ServiceBuilder;
use tower_http::compression::{
  predicate::{NotForContentType, Predicate, SizeAbove},
  CompressionLayer,
};

genv::def!(PORT:u16 | 5123);

// json: Bytes
async fn index() -> aerr::msg!() {
  // let subject;
  // let txt;
  // let status;
  // match sonic_rs::from_slice::<mail::Root>(&json) {
  //   Err(e) => {
  //     txt = String::from_utf8_lossy(&json);
  //     let e = e.to_string();
  //     subject = format!("mailhook json parse error : {}", &e);
  //     tracing::error!("{}\n{}", e, &txt);
  //     status = StatusCode::BAD_REQUEST;
  //   }
  //   Ok(root) => {
  //     status = StatusCode::OK;
  //     let payload = root.payload;
  //     subject = payload.subject;
  //     txt = payload.txt.into();
  //   }
  // }
  // if status == StatusCode::OK {
  //   Ok(())
  // } else {
  //   aerr::err(status, ())
  // }

  Ok(sonic_rs::to_string(&alive::status().await?)?)
}

pub static TEXT_JSON: &'static HeaderValue = HeaderValue::from_static("text/json");

async fn header(req: Request<Body>, next: Next) -> impl IntoResponse {
  let mut res = next.run(req).await;
  res
    .headers_mut()
    .insert(http::header::CONTENT_TYPE, TEXT_JSON);
  res
}

#[tokio::main]
async fn main() -> Result<()> {
  loginit::init();
  let predicate = SizeAbove::new(256)
    .and(NotForContentType::GRPC)
    .and(NotForContentType::IMAGES);

  let app = Router::new()
    .route("/", get(aerr::FnAny(index)))
    .layer(CompressionLayer::new().compress_when(predicate))
    .layer(ServiceBuilder::new().layer(middleware::from_fn(header)));
  let addr = SocketAddr::from(([0, 0, 0, 0], PORT()));

  tracing::info!("http://{}", addr);

  let listener = tokio::net::TcpListener::bind(addr).await?;
  axum::serve(listener, app).await?;

  Ok(())
}
