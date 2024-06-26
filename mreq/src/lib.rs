use aok::Result;
use bytes::Bytes;
use citer::CIter;
use reqwest::{header::HeaderMap, Body, Method, Request, StatusCode};

pub struct Mreq {
  pub host_li: Vec<String>,
  pub headers: HeaderMap,
  pub pos: usize,
}

impl Mreq {
  pub fn new<S: Into<String>>(host_li: impl IntoIterator<Item = S>, headers: HeaderMap) -> Self {
    Self {
      host_li: host_li.into_iter().map(|s| s.into()).collect(),
      headers,
      pos: 0,
    }
  }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("{url} : {msg}")]
  InternalServer { url: String, msg: String },
}

impl Mreq {
  pub async fn execute(
    &mut self,
    method: Method,
    url: impl Into<String>,
    build: impl FnOnce(&mut Request),
  ) -> Result<Bytes> {
    let mut host_iter = CIter::new(&self.host_li[..], self.pos);

    let mut host = host_iter.next().unwrap();
    let url = url.into();
    let mut req = reqwest::Request::new(method, format!("https://{host}/{url}").parse().unwrap());
    *req.headers_mut() = self.headers.clone();
    build(&mut req);

    loop {
      let r = ireq::REQ.execute(req.try_clone().unwrap());
      if let Some(h) = host_iter.next() {
        host = h;
        if let Ok(r) = xerr::ok!(r.await) {
          let status = r.status();
          if status.is_success() {
            if let Ok(bin) = xerr::ok!(r.bytes().await) {
              self.pos = host_iter.pos();
              return Ok(bin);
            }
          } else {
            let url = r.url().to_string();
            if status == StatusCode::INTERNAL_SERVER_ERROR {
              return Err(
                Error::InternalServer {
                  url,
                  msg: r.text().await?,
                }
                .into(),
              );
            } else {
              tracing::warn!("❌ {} : {} ", url, status);
            }
          }
        }
        req.url_mut().set_host(Some(host))?;
      } else {
        self.pos = host_iter.pos();
        return Ok(r.await?.bytes().await?);
      }
    }
  }

  pub async fn get(&mut self, url: impl Into<String>) -> Result<Bytes> {
    self.execute(Method::GET, url, |_| {}).await
  }

  pub async fn post(&mut self, url: impl Into<String>, body: impl Into<Body>) -> Result<Bytes> {
    let body = body.into();
    self
      .execute(Method::POST, url, |req| {
        *req.body_mut() = Some(body);
      })
      .await
  }
}
