#![feature(async_closure)]
mod index;
use index::index;
mod ping;
use std::{net::SocketAddr, time::Duration};

use aok::Result;
use axum::{
  body::Body,
  error_handling::HandleErrorLayer,
  extract::Request,
  http::{self, HeaderValue, StatusCode},
  middleware,
  middleware::Next,
  response::IntoResponse,
  routing::get,
  BoxError, Router,
};
use ping::ping;
use tower::ServiceBuilder;
use tower_http::compression::{
  predicate::{NotForContentType, Predicate, SizeAbove},
  CompressionLayer,
};

genv::def!(PORT:u16 | 5123);
genv::s!(HEALTHCHECK);

pub static TEXT_JSON: &str = "text/json";

async fn header(req: Request<Body>, next: Next) -> impl IntoResponse {
  let mut res = next.run(req).await;
  res.headers_mut().insert(
    http::header::CONTENT_TYPE,
    HeaderValue::from_static(TEXT_JSON),
  );
  res
}

const TIMEOUT: u64 = 600;

#[tokio::main]
async fn main() -> Result<()> {
  tokio::spawn(async {
    let healthcheck = HEALTHCHECK.as_str();
    alive::cron::run(async move || {
      xerr::log!(ireq::get(healthcheck).await);
    })
    .await;
  });

  let middleware = ServiceBuilder::new()
    .layer(HandleErrorLayer::new(|error: BoxError| async move {
      if error.is::<tower::timeout::error::Elapsed>() {
        Ok((StatusCode::REQUEST_TIMEOUT, "timeout"))
      } else {
        Err((
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Internal Error: {}", error),
        ))
      }
    }))
    .timeout(Duration::from_secs(TIMEOUT).into())
    .layer(ServiceBuilder::new());

  loginit::init();
  let predicate = SizeAbove::new(256)
    .and(NotForContentType::GRPC)
    .and(NotForContentType::IMAGES);

  let app = Router::new()
    .route("/", get(aerr::FnAny(index)))
    .route("/ping", get(aerr::FnAny(ping)))
    .layer(middleware)
    .layer(CompressionLayer::new().compress_when(predicate))
    .layer(ServiceBuilder::new().layer(middleware::from_fn(header)));
  let addr = SocketAddr::from(([0, 0, 0, 0], PORT()));

  tracing::info!("http://{}", addr);

  axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

  Ok(())
}
