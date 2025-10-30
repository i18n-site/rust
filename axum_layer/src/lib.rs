#![feature(impl_trait_in_assoc_type)]

use axum::{Router, http::StatusCode};

pub mod log;

pub fn use_compress(router: Router) -> Router {
  use tower_http::{
    CompressionLevel,
    compression::{
      CompressionLayer,
      predicate::{NotForContentType, Predicate, SizeAbove},
    },
  };

  router.layer(
    CompressionLayer::new()
      .br(true)
      .zstd(true)
      .quality(CompressionLevel::Precise(16))
      .compress_when(SizeAbove::new(512)
        // still don't compress gRPC
        // .and(NotForContentType::GRPC)
        // still don't compress images
        .and(NotForContentType::IMAGES)),
  )
}

genv::def!(AXUM_TIMEOUT:u64 | 600);

pub fn use_timeout(router: Router) -> Router {
  use axum::error_handling::HandleErrorLayer;
  use tower::{BoxError, ServiceBuilder};

  let middleware = ServiceBuilder::new()
    .layer(HandleErrorLayer::new(|error: BoxError| async move {
      if error.is::<tower::timeout::error::Elapsed>() {
        Ok((StatusCode::REQUEST_TIMEOUT, "timeout"))
      } else {
        Err((
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Internal Error: {error}"),
        ))
      }
    }))
    .layer(crate::log::Log)
    .timeout(std::time::Duration::from_secs(AXUM_TIMEOUT()))
    .layer(ServiceBuilder::new());

  router.layer(middleware.into_inner())
}

pub fn layer(router: Router) -> Router {
  let router = use_compress(router);

  use_timeout(router)
}
