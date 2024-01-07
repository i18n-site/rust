use std::{future::Future, time::Duration};

use reqwest::{Client, Response};

genv::def!(IPV6_PROXY);

pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(8);

pub const TIMEOUT: Duration = Duration::from_secs(120);

thread_local! {
    static CLIENT : Client =  client().build().unwrap();
}

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

pub fn proxy(
  build: impl FnOnce(&ClientWithMiddleware) -> RequestBuilder,
) -> impl Future<Output = Result<Response, reqwest_middleware::Error>> {
  let client = retry_client(
    client_builder()
      .proxy(reqwest::Proxy::https(proxy_next()).unwrap())
      .build()
      .unwrap(),
  );
  build(&client).send()
}

pub fn send(
  build: impl FnOnce(&ClientWithMiddleware) -> RequestBuilder,
) -> impl Future<Output = Result<Response, reqwest_middleware::Error>> {
  CLIENT.with(|client| build(client).send())
}
