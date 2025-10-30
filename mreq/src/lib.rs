use aok::Result;
use bytes::Bytes;
use citer::CIter;
use header_map::header_map;
use reqwest::{Body, Method, Request, StatusCode, header::HeaderMap};

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

  #[error("request is empty")]
  RequestEmpty,

  #[error("{code} {url}\n{msg}")]
  Status {
    code: StatusCode,
    url: String,
    msg: String,
  },
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

genv::s!(MREQ_PROTOCOL:String|"https".into());

impl Mreq {
  pub async fn execute(
    &mut self,
    method: Method,
    url_suffix: impl Into<String>,
    build: impl FnOnce(&mut Request),
  ) -> Result<Bytes> {
    let mut host_iter = CIter::new(&self.host_li[..], self.pos);

    if let Some(host) = host_iter.next() {
      let url_suffix = format!("/{}", url_suffix.into());
      let url = format!("{}://{host}{url_suffix}", &*MREQ_PROTOCOL);
      let mut req = Request::new(method, url.parse()?);
      *req.headers_mut() = self.headers.clone();
      build(&mut req);

      loop {
        if let Some(r) = req.try_clone() {
          match ireq::REQ.execute(r).await {
            Ok(r) => {
              let status = r.status();
              if status.is_success() {
                self.pos = host_iter.pos();
                if let Ok(bin) = xerr::ok!(r.bytes().await) {
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
                      url,
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
                  let headers = r
                    .headers()
                    .into_iter()
                    .map(|(k, v)| format!("  {k}: {}", v.to_str().unwrap_or_default()))
                    .collect::<Vec<_>>()
                    .join("\n");
                  let msg = r.text().await.unwrap_or_default();
                  if let Some(host) = host_iter.next() {
                    tracing::warn!("\n⚠ {} {}\n{}\n{}", status, url, headers, msg);
                    req.url_mut().set_host(Some(host))?;
                  } else {
                    self.pos = host_iter.pos();
                    return Err(
                      Error::Status {
                        url,
                        code: status,
                        msg,
                      }
                      .into(),
                    );
                  }
                }
              }
            }
            Err(err) => {
              tracing::warn!("{} {}", req.url(), err);
              if let Some(host_path) = host_iter.next() {
                let url = req.url_mut();
                let host = if let Some(p) = host_path.find('/')
                  && (1 + p) < host_path.len()
                {
                  url.set_path(&format!("{}{url_suffix}", &host[p + 1..]));
                  &host[..p]
                } else {
                  url.set_path(&url_suffix);
                  host_path.as_ref()
                };
                url.set_host(Some(host))?;
              } else {
                self.pos = host_iter.pos();
                return Err(err.into());
              }
            }
          }
        } else {
          return Err(Error::RequestEmpty.into());
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
