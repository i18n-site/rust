#![cfg_attr(docsrs, feature(doc_cfg))]

use std::net::SocketAddr;

use err_exit::err_exit;

#[cfg(feature = "http")]
mod http;

#[cfg(feature = "http")]
pub use http::http;

#[cfg(feature = "grpc")]
mod grpc;

#[cfg(feature = "grpc")]
pub use grpc::grpc;

pub async fn init() {
  log_init::init();
  match static_::init().await {
    Ok(()) => (),
    Err(err) => err_exit!("static init error: {err}"),
  }
}

pub fn env_addr(name: &str, default: u16) -> SocketAddr {
  macro_rules! default {
    () => {
      SocketAddr::from(([0, 0, 0, 0], default))
    };
  }
  let addr = if let Ok(addr) = std::env::var(name) {
    match addr.parse() {
      Ok(addr) => addr,
      Err(err) => {
        log::error!("{name}={addr} ADDR PARSE ERROR: {err}");
        default!()
      }
    }
  } else {
    default!()
  };
  log::info!("{name} {addr}");
  addr
}

pub type Result<T = (), E = Box<dyn std::error::Error + Send + Sync>> = std::result::Result<T, E>;
