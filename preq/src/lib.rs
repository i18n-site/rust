use std::{future::Future, time::Duration};

use reqwest::{Client, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

genv::def!(IPV6_PROXY);

pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(8);

pub const TIMEOUT: Duration = Duration::from_secs(300);

thread_local! {
    static CLIENT :ClientWithMiddleware= retry_client(Client::builder().brotli(true).build().unwrap());
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

pub fn retry_client(client: Client) -> ClientWithMiddleware {
  ClientBuilder::new(client)
    .with(RetryTransientMiddleware::new_with_policy(
      ExponentialBackoff::builder()
        .retry_bounds(Duration::from_millis(1), Duration::from_secs(2))
        .build_with_max_retries(9),
    ))
    .build()
}

pub fn proxy(
  build: impl FnOnce(&ClientWithMiddleware) -> RequestBuilder,
) -> impl Future<Output = Result<Response, reqwest_middleware::Error>> {
  let client = retry_client(
    Client::builder()
        .brotli(true)
        // .http3_prior_knowledge()
        .proxy(reqwest::Proxy::https(proxy_next()).unwrap())
        .timeout(TIMEOUT)
        .connect_timeout(CONNECT_TIMEOUT)
        .build().unwrap(),
  );
  build(&client).send()
}

pub fn send(
  build: impl FnOnce(&ClientWithMiddleware) -> RequestBuilder,
) -> impl Future<Output = Result<Response, reqwest_middleware::Error>> {
  CLIENT.with(|client| build(client).send())
}
