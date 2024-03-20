#![feature(async_closure)]
mod li;
mod ping;

use aok::{Result, OK};
use axum::{
  body::Body,
  extract::Request,
  http::{header, HeaderValue},
  middleware,
  middleware::Next,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use ping::ping;
use tower::ServiceBuilder;

genv::def!(PORT:u16 | 5123);
genv::s!(HEALTHCHECK);

async fn header(req: Request<Body>, next: Next) -> impl IntoResponse {
  let origin = req
    .headers()
    .get(header::ORIGIN)
    .map(|i| i.to_str().map(|s| s.to_owned()));
  let mut res = next.run(req).await;

  let headers = res.headers_mut();
  headers.insert(
    header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
    HeaderValue::from_static("true"),
  );
  if let Some(Ok(origin)) = origin {
    headers.insert(
      header::ACCESS_CONTROL_ALLOW_ORIGIN,
      HeaderValue::from_str(&origin).unwrap(),
    );
  }
  headers.insert(
    header::ACCESS_CONTROL_ALLOW_METHODS,
    HeaderValue::from_static("*"),
  );
  // headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("content-type"));

  // res
  //   .headers_mut()
  //   .insert(AccessControlAllowHeaders::from("content-type"));

  res
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
  loginit::init();

  tokio::spawn(async {
    let healthcheck = HEALTHCHECK.as_str();
    alive::cron::run(|| async move {
      xerr::log!(ireq::get(healthcheck).await);
    })
    .await;
  });

  let app = Router::new()
    .route("/Li", post(re::FnAny(li::post)))
    .route("/ping", get(re::FnAny(ping)))
    .layer(ServiceBuilder::new().layer(middleware::from_fn(header)));

  t3::srv(app, PORT()).await;

  OK
}
