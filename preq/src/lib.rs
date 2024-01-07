use std::{future::Future, time::Duration};

use reqwest::{Client, IntoUrl, Response};

genv::def!(IPV6_PROXY);

pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(8);

pub const TIMEOUT: Duration = Duration::from_secs(120);

#[static_init::dynamic]
pub static CLIENT: Client = client().build().unwrap();

#[static_init::dynamic]
static PROXY: Vec<String> = IPV6_PROXY::<String>()
  .split(' ')
  .map(|i| format!("http://{i}"))
  .collect();

static mut N: usize = 0;

pub fn proxy_next() -> &'static str {
  &PROXY[unsafe {
    N = (N + 1) % PROXY.len();
    N
  }]
}

pub fn client() -> reqwest::ClientBuilder {
  Client::builder()
        .brotli(true)
        // .http3_prior_knowledge()
        .timeout(TIMEOUT)
        .connect_timeout(CONNECT_TIMEOUT)
}

pub fn proxy(proxy_url: impl IntoUrl) -> reqwest::Client {
  // let client = retry_client(
  client()
    .proxy(reqwest::Proxy::https(proxy_url).unwrap())
    .build()
    .unwrap()
  // proxy_next()
  // build(&client).send()
}

// pub fn send(
//   build: impl FnOnce(&ClientWithMiddleware) -> RequestBuilder,
// ) -> impl Future<Output = Result<Response, reqwest_middleware::Error>> {
//   CLIENT.with(|client| build(client).send())
// }
