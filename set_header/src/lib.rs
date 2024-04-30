use std::{ops::Deref, sync::Arc};

use axum::{
  extract::Request,
  http::{header::CONTENT_TYPE, HeaderName, HeaderValue},
  middleware::Next,
  response::Response,
};
use parking_lot::Mutex;

#[derive(Default, Debug, Clone)]
pub struct _Header(pub Vec<(HeaderName, String)>);

impl Deref for _Header {
  type Target = Vec<(HeaderName, String)>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Default, Debug, Clone)]
pub struct Header(pub Arc<Mutex<_Header>>);

impl Deref for Header {
  type Target = Mutex<_Header>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Header {
  pub fn push(&self, header_name: HeaderName, header_value: String) {
    self.lock().0.push((header_name, header_value))
  }
}

async fn _set_header(mut req: Request, next: Next) -> anyhow::Result<Response> {
  let header = Header::default();
  req.extensions_mut().insert(header.clone());
  let mut r = next.run(req).await;

  let mut has_content_type = false;
  {
    if let Ok(header) = Arc::try_unwrap(header.0) {
      let header = header.into_inner().0;
      for (k, v) in header {
        if k == CONTENT_TYPE {
          has_content_type = true;
        }
        match v.parse() {
          Ok(v) => {
            r.headers_mut().append(k, v);
          }
          Err(err) => {
            tracing::error!("{}", err);
          }
        }
      }
    }
  }
  if !has_content_type {
    // for cloudflare compress : https://developers.cloudflare.com/speed/optimization/content/brotli/content-compression/
    r.headers_mut()
      .insert(CONTENT_TYPE, HeaderValue::from_static("text/js"));
  }
  Ok(r)
}

pub async fn set_header(req: Request, next: Next) -> Response {
  amid::middleware(_set_header(req, next).await)
}
