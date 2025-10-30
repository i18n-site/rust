#![cfg_attr(docsrs, feature(doc_cfg))]

mod call;
mod response;
mod result;

use std::{
  error::Error,
  fmt::{Debug, Display},
};

pub use call::{AsyncCall, Call, Func, ParseResult, ReqArgs};
pub use response::Response;
pub use result::Result;

pub trait Map {
  fn get(&self, key: impl AsRef<str>) -> Option<&str>;
}

pub trait Init: Sized {
  fn init(m: &impl Map) -> impl Future<Output = anyhow::Result<Self>>;
}

pub trait ExtVal: 'static + Send + Sync + Clone {}

pub trait Ext {
  fn ext<T: ExtVal + Init>(&self, headers: &impl Map) -> impl Future<Output = anyhow::Result<T>>;
}

pub struct HeadersExt<H: Map, E: Ext> {
  pub headers: H,
  pub ext: E,
}

pub trait Req: Map {
  fn ext<T: ExtVal + Init>(&self) -> impl Future<Output = anyhow::Result<T>>;
}

impl<H: Map, E: Ext> Map for HeadersExt<H, E> {
  fn get(&self, key: impl AsRef<str>) -> Option<&str> {
    self.headers.get(key)
  }
}

impl<H: Map, E: Ext> Req for HeadersExt<H, E> {
  fn ext<T: ExtVal + Init>(&self) -> impl Future<Output = anyhow::Result<T>> {
    self.ext.ext(&self.headers)
  }
}

#[derive(Debug)]
pub struct ParseError {
  pub bytes: bytes::Bytes,
  pub err: anyhow::Error,
}

impl Error for ParseError {}

impl Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.err)
  }
}

pub trait IntoResponse<T> {
  fn into_response(self) -> T;
}

pub trait Log {
  fn info<T: Debug>(name: &str, req: &impl Req, args: T, cost: u64);
  fn response<T: Debug>(name: &str, req: &impl Req, args: T, cost: u64, response: &Response);
  fn error<T: Debug>(name: &str, req: &impl Req, args: T, cost: u64, err: &anyhow::Error);
  fn args_parse(name: &str, req: &impl Req, err: &ParseError);
}

#[cfg(feature = "volo")]
pub mod volo;
