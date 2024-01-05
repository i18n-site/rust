#![feature(async_closure)]
mod index;
use index::index;
mod ping;

use aok::{Result, OK};
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
use ping::ping;
use tower::ServiceBuilder;

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

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
  t3::init();

  tokio::task::spawn_blocking(async || {
    let healthcheck = HEALTHCHECK.as_str();
    alive::cron::run(async move || {
      xerr::log!(ireq::get(healthcheck).await);
    })
    .await;
  });

  let app = Router::new()
    .route("/", get(re::FnAny(index)))
    .route("/ping", get(re::FnAny(ping)))
    .layer(ServiceBuilder::new().layer(middleware::from_fn(header)));

  t3::srv(app).await;

  OK
}
