use aok::Result;
use reqwest::{Body, Client, IntoUrl, RequestBuilder, StatusCode, redirect::Policy};
use serde::de::DeserializeOwned;

use crate::Error;

pub async fn post<R: DeserializeOwned>(
  url: impl AsRef<str>,
  body: impl Into<String>,
  headers: impl Fn(RequestBuilder) -> RequestBuilder,
) -> Result<R> {
  let url = url.as_ref();
  let body = body.into();
  let req = ireq::REQ.post(url).body(body.clone());
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
