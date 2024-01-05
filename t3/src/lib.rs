#![feature(min_specialization)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

pub mod origin;
pub use origin::{origin, origin_tld};
pub mod host;
pub use axum::{
  self,
  extract::{ConnectInfo, Path},
  http::{header::HeaderMap, StatusCode},
  response::IntoResponse,
  Extension,
};
pub use host::{host, host_tld};
pub use tower_http;
pub use tracing;
mod log;
mod srv;

#[macro_export]
macro_rules! api {
  () => {
    pub mod api {
      include!(concat!(env!("OUT_DIR"), "/api.rs"));
    }
  };
}

pub use log::init;
pub use srv::srv;

pub type Response = re::Result<axum::response::Response>;

pub type E<T> = Extension<T>;

pub fn ok() -> Response {
  let r = (StatusCode::OK, "").into_response();
  Ok(r)
}

#[macro_export]
macro_rules! compression_layer {
  () => {{
    use tower_http::compression::{
        predicate::{NotForContentType, Predicate, SizeAbove},
        CompressionLayer,
    };
    let predicate = SizeAbove::new(256)
        // still don't compress gRPC
        .and(NotForContentType::GRPC)
        // still don't compress images
        .and(NotForContentType::IMAGES);
    CompressionLayer::new().compress_when(predicate)
  }};
}
