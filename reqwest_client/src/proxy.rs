use reqwest::{ClientBuilder, Proxy};

pub fn proxy(proxy: Proxy) -> ClientBuilder {
  crate::client()
    .proxy(proxy)
    .danger_accept_invalid_certs(true)
}
