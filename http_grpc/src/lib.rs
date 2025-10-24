#![cfg_attr(docsrs, feature(doc_cfg))]

use std::marker::PhantomData;

use bytes::Bytes;
use pilota::pb::Message;

include!(concat!(env!("OUT_DIR"), "/api.rs"));

pub struct ResData<T> {
  pub code: u32,
  pub body: Option<Bytes>,
  _t: PhantomData<T>,
}

pub struct Res {
  pub id: u64,
}

impl Res {
  pub fn dump<T>(&self, res: impl Into<ResData<T>>) -> Bytes {
    let res = res.into();
    let mut body = Default::default();
    api::http_grpc::Response {
      id: self.id,
      code: res.code,
      body: res.body,
    }
    .encode_length_delimited(&mut body)
    .unwrap();
    body.into_bytes_mut().into()
  }
}

impl<T: Message> From<T> for ResData<T> {
  fn from(t: T) -> Self {
    let mut body = Default::default();
    t.encode(&mut body).unwrap();
    Self {
      code: 0,
      body: if body.is_empty() {
        None
      } else {
        Some(body.into_bytes_mut().into())
      },
      _t: PhantomData,
    }
  }
}

impl<T: Message> From<anyhow::Result<T>> for ResData<T> {
  fn from(t: std::result::Result<T, anyhow::Error>) -> Self {
    match t {
      Ok(t) => t.into(),
      Err(err) => {
        let code;
        let body;
        match err.downcast::<xrpc::Status>() {
          Ok(status) => {
            code = status.code.as_u16() as _;
            body = status.body.into();
          }
          Err(err) => {
            code = 500;
            let err = err.to_string();
            body = err.into();
          }
        }
        Self {
          code,
          body: Some(body),
          _t: PhantomData,
        }
      }
    }
  }
}
