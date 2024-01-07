use std::time::Duration;

use reqwest::{Client, IntoUrl};

pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(8);

pub const TIMEOUT: Duration = Duration::from_secs(120);

genv::s!(IPV6_PROXY_TOKEN);

pub fn proxy(url: impl IntoUrl) -> reqwest::Client {
  Client::builder()
        .proxy(reqwest::Proxy::https(url).unwrap())
        .brotli(true)
        // .http3_prior_knowledge()
        .timeout(TIMEOUT)
        .connect_timeout(CONNECT_TIMEOUT).build().unwrap()
}

pub async fn post(
  client: &reqwest::Client,
  url: impl IntoUrl,
  body: impl Into<reqwest::Body>,
) -> reqwest::Result<reqwest::Response> {
  client
    .post(url)
    .header("T", &*IPV6_PROXY_TOKEN)
    .body(body)
    .send()
    .await
}

genv::def!(IPV6_PROXY, IPV6_PROXY_PORT);

// #[static_init::dynamic]
// static PROXY: Vec<String> = IPV6_PROXY::<String>()
//   .split(' ')
//   .map(|i| format!("http://{i}"))
//   .collect();
//
// static mut N: usize = 0;
//
// pub fn proxy_next() -> &'static str {
//   &PROXY[unsafe {
//     N = (N + 1) % PROXY.len();
//     N
//   }]
// }
