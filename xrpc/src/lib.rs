#![cfg_attr(docsrs, feature(doc_cfg))]

mod call;
mod response;
mod result;

pub use call::{AsyncCall, Call};
pub use http::Extensions;
pub use response::Response;
pub use result::Result;

pub trait Map {
  fn get(&self, key: impl AsRef<str>) -> Option<&str>;
}

pub trait Req {
  fn headers(&self) -> impl Map;
  fn extensions(&self) -> &Extensions;
  fn extensions_mut(&mut self) -> &mut Extensions;
}

pub trait IntoResponse<T> {
  fn into_response(self) -> T;
}

#[cfg(feature = "volo")]
pub mod volo;
