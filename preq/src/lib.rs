use std::time::Duration;

use reqwest::{Client, IntoUrl, RequestBuilder};

pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(8);

pub const TIMEOUT: Duration = Duration::from_secs(120);

genv::s!(IPV6_PROXY_USER, IPV6_PROXY_PASSWD);

genv::def!(IPV6_PROXY, IPV6_PROXY_PORT);

pub fn proxy(url: impl AsRef<str>) -> reqwest::Client {
  let url = format!(
    "http://{}:{}@{}",
    *IPV6_PROXY_USER,
    *IPV6_PROXY_PASSWD,
    url.as_ref()
  );
  Client::builder()
        .proxy(reqwest::Proxy::https(url).unwrap())
        .brotli(true)
        // .http3_prior_knowledge()
        .timeout(TIMEOUT)
        .connect_timeout(CONNECT_TIMEOUT).build().unwrap()
}

pub async fn post(
  client: &[reqwest::Client],
  url: impl IntoUrl,
  build: impl FnOnce(RequestBuilder) -> RequestBuilder,
) -> reqwest::Result<reqwest::Response> {
  let mut retry = 3;
  let mut pos = 0;
  let url = url.into_url()?;
  loop {
    let r = build(client[pos].post(url.clone())).send().await;
    if retry == 0 {
      return r;
    }
    if let Ok(r) = r {
      return Ok(r);
    }
    pos = (pos + 1) % client.len();
    retry -= 1;
  }
}

pub async fn post_form(
  client: &[reqwest::Client],
  url: impl IntoUrl,
  form: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
) -> reqwest::Result<reqwest::Response> {
  let url = url.into_url()?;
  post(client, url, |req| {
    let form = form
      .into_iter()
      .map(|(k, v)| {
        format!(
          "{}={}",
          k.as_ref(),
          form_urlencoded::byte_serialize(v.as_ref().as_bytes()).collect::<String>()
        )
      })
      .collect::<Vec<_>>()
      .join("&");

    req
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(form)
  })
  .await
}

// body: impl Into<reqwest::Body>,

/*
  let form = all
    .iter()
    .map(|i| {
      format!(
        "q={}",
        form_urlencoded::byte_serialize(i.as_ref().as_bytes()).collect::<String>()
      )
    })
    .collect::<Vec<_>>()
    .join("&");
"https://translate.google.com/translate_a/t?client=gtx&tl=en&sl=zh"
header("Content-Type", "application/x-www-form-urlencoded")
body(form)
*/

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
