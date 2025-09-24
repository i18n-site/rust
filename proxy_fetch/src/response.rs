use std::fmt;

use bytes::Bytes;
use reqwest::{header::HeaderMap, StatusCode};

/// Represents an HTTP response, containing status, headers, and body.
///
/// It can be used as a success value or as an error.
///
/// 代表一个HTTP响应，包含状态、头部和正文。
///
/// 它既可以作为成功值，也可以作为错误。
#[derive(Debug)]
pub struct Response {
  pub status: StatusCode,
  pub headers: HeaderMap,
  pub body: Bytes,
}

impl fmt::Display for Response {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Response status: {}", self.status)
  }
}

impl std::error::Error for Response {}
