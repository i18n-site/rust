#![cfg_attr(docsrs, feature(doc_cfg))]

use std::time::Duration;

use reqwest::{Client, ClientBuilder};

#[cfg(any(feature = "client", feature = "proxy"))]
genv::s!(TIMEOUT: u64 | 60);

genv::s!(USER_AGENT: String | String::from("curl/8.1.1"));

pub fn client() -> ClientBuilder {
  Client::builder()
    .user_agent(USER_AGENT.as_str())
    .zstd(true)
    .gzip(true)
    .brotli(true)
    .connect_timeout(Duration::from_secs(9))
}

#[cfg(feature = "client")]
#[static_init::dynamic]
pub static CLIENT: Client = client()
  .timeout(Duration::from_secs(*TIMEOUT))
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
