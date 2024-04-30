use std::net::{IpAddr, Ipv6Addr, SocketAddr};

use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Router};
use coarsetime::Duration;
use tower::ServiceBuilder;
use tower_http::{
  self,
  compression::{
    predicate::{NotForContentType, Predicate, SizeAbove},
    CompressionLayer,
  },
};

const TIMEOUT: u64 = 600;

pub async fn srv_addr(router: Router, addr: SocketAddr) {
  tracing::info!("http://{addr}");
  let bind = tokio::net::TcpListener::bind(addr).await.unwrap();
  axum::serve(
    bind,
    router.into_make_service_with_connect_info::<SocketAddr>(),
  )
  .await
  .expect("failed");
}

pub async fn srv_catch(router: Router, addr: SocketAddr) {
  // [How to set timeouts ?](https://github.com/tokio-rs/axum/discussions/1383)
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

  srv_addr(
    router.layer(middleware.into_inner()).layer(
      CompressionLayer::new()
        .zstd(true)
        .br(true)
        .quality(CompressionLevel::Best)
        .compress_when(
          SizeAbove::new(256)
        // still don't compress gRPC
        .and(NotForContentType::GRPC)
        // still don't compress images
        .and(NotForContentType::IMAGES),
        ),
    ),
    addr,
  )
  .await;
}

pub async fn srv(router: Router, port: u16) -> u16 {
  let addr = SocketAddr::new(IpAddr::from(Ipv6Addr::UNSPECIFIED), port);
  srv_catch(router, addr).await;
  port
}
