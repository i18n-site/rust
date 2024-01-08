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

pub const MAX_RETRY: usize = 3;

pub async fn post(
  n: usize,
  client_li: &[reqwest::Client],
  url: impl IntoUrl,
  build: impl Fn(RequestBuilder) -> RequestBuilder,
) -> reqwest::Result<reqwest::Response> {
  let mut retry = 0;
  let url = url.into_url()?;
  loop {
    dbg!(retry);
    let client = &client_li[(n.overflowing_add(retry)).0 % client_li.len()];
    let r = build(client.post(url.clone())).send().await;
    retry += 1;
    if retry >= MAX_RETRY {
      return r;
    }
    if let Ok(r) = xerr::ok!(r) {
      return Ok(r);
    }
  }
}

pub async fn post_form(
  n: usize,
  client_li: &[reqwest::Client],
  url: impl IntoUrl,
  form: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
) -> reqwest::Result<reqwest::Response> {
  let url = url.into_url()?;
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
  post(n, client_li, url, |req| {
    req
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(form.clone())
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

static mut N: usize = 0;

pub struct Proxy(Vec<reqwest::Client>);

#[static_init::dynamic]
pub static mut PROXY: Proxy = {
  let mut v = Vec::new();
  let port: u16 = IPV6_PROXY_PORT();
  for i in IPV6_PROXY::<String>().split(' ') {
    v.push(proxy(format!("{i}:{port}")));
  }
  Proxy(v)
};
impl Proxy {
  pub async fn post_form(
    &self,
    url: impl IntoUrl,
    form: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
  ) -> reqwest::Result<reqwest::Response> {
    unsafe { (N, _) = N.overflowing_add(1) };
    post_form(unsafe { N }, &self.0, url, form).await
  }
}
