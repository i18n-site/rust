use aok::Result;
use bytes::Bytes;
use citer::CIter;
use header_map::header_map;
use reqwest::{header::HeaderMap, Body, Method, Request, StatusCode};

pub struct Mreq {
  pub host_li: Vec<String>,
  pub headers: HeaderMap,
  pub pos: usize,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("{url} : {msg}")]
  InternalServer { url: String, msg: String },
  #[error("host li is empty")]
  HostLiEmpty,
  #[error("request is empty : {0:?}")]
  RequestEmpty(Request),
  #[error("{code} : {msg}")]
  Status { code: StatusCode, msg: String },
}

impl Mreq {
  pub fn new<'a, S: Into<String>>(
    host_li: impl IntoIterator<Item = S>,
    headers: impl IntoIterator<Item = (&'a str, &'a str)>,
  ) -> Self {
    Self {
      host_li: host_li.into_iter().map(|s| s.into()).collect(),
      headers: header_map(headers),
      pos: 0,
    }
  }
}

impl Mreq {
  pub async fn execute(
    &mut self,
    method: Method,
    url: impl Into<String>,
    build: impl FnOnce(&mut Request),
  ) -> Result<Bytes> {
    let mut host_iter = CIter::new(&self.host_li[..], self.pos);

    if let Some(mut host) = host_iter.next() {
      let url = url.into();
      let mut req = Request::new(method, format!("https://{host}/{url}").parse()?);
      *req.headers_mut() = self.headers.clone();
      build(&mut req);

      loop {
        if let Some(r) = req.try_clone() {
          let r = ireq::REQ.execute(r);
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
                if [
                  StatusCode::UNAUTHORIZED,
                  //  StatusCode::NOT_FOUND
                ]
                .contains(&status)
                {
                  return Err(
                    Error::Status {
                      code: status,
                      msg: r.text().await.unwrap_or_default(),
                    }
                    .into(),
                  );
                }
                if status == StatusCode::INTERNAL_SERVER_ERROR {
                  return Err(
                    Error::InternalServer {
                      url,
                      msg: r.text().await.unwrap_or_default(),
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
        } else {
          return Err(Error::RequestEmpty(req).into());
        }
      }
    } else {
      Err(Error::HostLiEmpty.into())
    }
  }

  pub async fn get(&mut self, url: impl Into<String>) -> Result<Bytes> {
    self.execute(Method::GET, url, |_| {}).await
  }

  pub async fn post_no_body(&mut self, url: impl Into<String>) -> Result<Bytes> {
    self.execute(Method::POST, url, |_| {}).await
  }

  pub async fn post<B: Into<Body>>(&mut self, url: impl Into<String>, body: B) -> Result<Bytes> {
    let body = body.into();
    self
      .execute(Method::POST, url, |req| {
        *req.body_mut() = Some(body);
      })
      .await
  }
}
