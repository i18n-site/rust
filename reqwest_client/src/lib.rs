use std::time::Duration;

use reqwest::{Client, ClientBuilder};

pub fn client() -> ClientBuilder {
  Client::builder()
    .user_agent("")
    .zstd(true)
    .gzip(true)
    .brotli(true)
    .connect_timeout(Duration::from_secs(9))
}

#[cfg(any(feature = "client", feature = "proxy"))]
pub const TIMEOUT: u64 = 120;

#[cfg(feature = "client")]
#[static_init::dynamic]
pub static CLIENT: Client = client()
  .timeout(Duration::from_secs(TIMEOUT))
  .build()
  .unwrap();

#[cfg(feature = "proxy")]
mod proxy;

#[cfg(feature = "proxy")]
pub use proxy::proxy;

#[cfg(feature = "proxy_from_env")]
mod proxy_from_env;

#[cfg(feature = "proxy_from_env")]
pub use proxy_from_env::{PROXY, ProxyIter, proxy_iter};
