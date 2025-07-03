use std::time::Duration;

use aok::Result;
pub use reqwest;
use reqwest::{Client, RequestBuilder, StatusCode, redirect::Policy};
use serde::de::DeserializeOwned;

use crate::Error;

#[static_init::dynamic]
pub static REQ: Client = {
  let b = Client::builder()
    .redirect(Policy::limited(6))
    .timeout(Duration::from_secs(6000))
    .zstd(true)
    .connect_timeout(Duration::from_secs(9));

  #[cfg(feature = "proxy")]
  let b = {
    use reqwest::Proxy;
    // export https_proxy=http://127.0.0.1:7890 http_proxy=http://127.0.0.1:7890 all_proxy=socks5://127.0.0.1:7890
    let mut b = b;
    if let Ok(url) = std::env::var("https_proxy") {
      b = b.proxy(Proxy::https(url).unwrap());
    }

    b
  };

  b.build().unwrap()
};

pub async fn post<R: DeserializeOwned>(
  url: impl AsRef<str>,
  body: impl Into<String>,
  headers: impl Fn(RequestBuilder) -> RequestBuilder,
) -> Result<R> {
  let url = url.as_ref();
  let body = body.into();
  let req = REQ.post(url).body(body.clone());
  let response = headers(req).send().await?;

  let status = response.status();
  let msg = response.text().await?;

  if status == StatusCode::OK {
    match sonic_rs::from_str(&msg) {
      Ok(r) => Ok(r),
      Err(err) => Err(Error::DecodeError { msg, err }.into()),
    }
  } else {
    Err(
      Error::RequestError {
        status,
        url: url.into(),
        msg,
      }
      .into(),
    )
  }
}
