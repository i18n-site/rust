use std::{
  pin::Pin,
  task::{self, Poll},
};

use hyper_util::{
  client::legacy::connect::{Connected, Connection},
  rt::TokioIo,
};
use tokio::io::{AsyncRead, AsyncWrite};

/// Generic proxy stream macro_enumsper
/// 通用代理流包装器
pub struct Stream<T>(pub TokioIo<T>);

impl<T: AsyncRead + AsyncWrite + Unpin> hyper::rt::Read for Stream<T> {
  fn poll_read(
    self: Pin<&mut Self>,
    cx: &mut task::Context<'_>,
    buf: hyper::rt::ReadBufCursor<'_>,
  ) -> Poll<Result<(), std::io::Error>> {
    Pin::new(&mut self.get_mut().0).poll_read(cx, buf)
  }
}

impl<T: AsyncRead + AsyncWrite + Unpin> hyper::rt::Write for Stream<T> {
  fn poll_write(
    self: Pin<&mut Self>,
    cx: &mut task::Context<'_>,
    buf: &[u8],
  ) -> Poll<Result<usize, std::io::Error>> {
    Pin::new(&mut self.get_mut().0).poll_write(cx, buf)
  }

  fn poll_flush(
    self: Pin<&mut Self>,
    cx: &mut task::Context<'_>,
  ) -> Poll<Result<(), std::io::Error>> {
    Pin::new(&mut self.get_mut().0).poll_flush(cx)
  }

  fn poll_shutdown(
    self: Pin<&mut Self>,
    cx: &mut task::Context<'_>,
  ) -> Poll<Result<(), std::io::Error>> {
    Pin::new(&mut self.get_mut().0).poll_shutdown(cx)
  }
}

impl<T: AsyncRead + AsyncWrite + Unpin> Connection for Stream<T> {
  fn connected(&self) -> Connected {
    Connected::new()
  }
}
