use std::net::SocketAddr;

use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Router};
use coarsetime::Duration;
use tower::ServiceBuilder;

genv::def!(PORT:u16|5123);

const TIMEOUT: u64 = 600;

pub async fn srv(router: Router) -> u16 {
  let port = PORT();
  let addr = SocketAddr::from(([0, 0, 0, 0], port));

  // https://github.com/tokio-rs/axum/discussions/1383
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
    .layer(crate::log::Log)
    .timeout(Duration::from_secs(TIMEOUT).into())
    .layer(ServiceBuilder::new());

  tracing::info!("http://{addr}");
  let bind = tokio::net::TcpListener::bind(addr).await.unwrap();
  axum::serve(
    bind,
    router
      .layer(crate::compression_layer!())
      .layer(middleware.into_inner())
      .into_make_service_with_connect_info::<SocketAddr>(),
  )
  .await
  .expect("failed");
  port
}
