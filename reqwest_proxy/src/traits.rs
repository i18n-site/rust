use std::io;

use hyper::Uri;
use hyper_util::client::legacy::connect::Connection;
use tower::Service;

/// A marker trait for streams that can be used by hyper.
/// 可供 hyper 使用的流的标记 Trait。
pub trait StreamTrait: hyper::rt::Read + hyper::rt::Write + Connection + Send + Unpin {}

impl<T> StreamTrait for T where T: hyper::rt::Read + hyper::rt::Write + Connection + Send + Unpin {}

/// A marker trait for connectors that can be used by the macro_enums.
/// 可供中间件使用的连接器的标记 Trait。
pub trait Conn:
  Service<Uri, Response = Self::Stream, Error = io::Error> + Clone + Send + Sync + 'static
where
  <Self as Service<Uri>>::Future: Send,
{
  type Stream: StreamTrait;
}

impl<T, S> Conn for T
where
  T: Service<Uri, Response = S, Error = io::Error> + Clone + Send + Sync + 'static,
  <T as Service<Uri>>::Future: Send,
  S: StreamTrait,
{
  type Stream = S;
}
