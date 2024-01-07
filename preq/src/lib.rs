use std::time::Duration;

use reqwest::{Client, IntoUrl};

pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(8);

pub const TIMEOUT: Duration = Duration::from_secs(120);

genv::s!(TOKEN);

#[static_init::dynamic]
pub static CLIENT: Client = client().build().unwrap();

pub fn client() -> reqwest::ClientBuilder {
  Client::builder()
        .brotli(true)
        // .http3_prior_knowledge()
        .timeout(TIMEOUT)
        .connect_timeout(CONNECT_TIMEOUT)
}

// reqwest::Proxy::https(proxy_url).unwrap()

pub async fn post(
  proxy: reqwest::Proxy,
  url: impl IntoUrl,
  body: impl Into<reqwest::Body>,
) -> reqwest::Result<reqwest::Response> {
  // let client = retry_client(
  let client = client().proxy(proxy).build()?;
  client.post(url).header("T", TOKEN).body(body).send().await
  // proxy_next()
  // build(&client).send()
}

// pub fn send(
//   build: impl FnOnce(&ClientWithMiddleware) -> RequestBuilder,
// ) -> impl Future<Output = Result<Response, reqwest_middleware::Error>> {
//   CLIENT.with(|client| build(client).send())
// }

genv::def!(IPV6_PROXY);

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
