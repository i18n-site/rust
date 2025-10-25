#![cfg_attr(docsrs, feature(doc_cfg))]

use bytes::Bytes;
use pilota::{LinkedBytes, pb::Message};

include!(concat!(env!("OUT_DIR"), "/api.rs"));

pub struct ResData {
  pub code: u32,
  pub body: Bytes,
}

pub struct Res {
  pub id: u64,
}

impl Res {
  pub fn dump(&self, res: impl Into<ResData>) -> Bytes {
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

pub fn call<T: xrpc::Call>(
  prefix: impl Into<String>,
  args: impl Into<Bytes>,
) -> xrpc::Result<<T as xrpc::Call>::Result> {
  match <T as xrpc::Call>::Args::decode(args.into()) {
    Ok(args) => T::call(prefix, &args),
    Err(err) => {
      todo!()
    }
  }
}

pub fn async_call<T: xrpc::AsyncCall>(
  prefix: impl Into<String>,
  args: impl Into<Bytes>,
) -> xrpc::Result<<T as xrpc::AsyncCall>::Result> {
  todo!()
}

impl<T: Message> From<xrpc::Result<T>> for ResData {
  fn from(t: xrpc::Result<T>) -> Self {
    use xrpc::Result;
    match t {
      Result::Ok(t) => {
        let mut body = LinkedBytes::with_capacity(t.encoded_len());
        match t.encode(&mut body) {
          Ok(_) => Self {
            code: 0,
            body: body.into_bytes_mut().into(),
          },
          Err(err) => Self {
            code: 500,
            body: err.to_string().into(),
          },
        }
      }
      Result::Err(err) => Self {
        code: 500,
        body: err.to_string().into(),
      },
      Result::Response(r) => Self {
        code: r.code as _,
        body: r.body,
      },
    }
  }
}
