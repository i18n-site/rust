use std::{
  io,
  pin::Pin,
  task::{Context, Poll},
};

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

/// A duplex stream that implements AsyncRead and AsyncWrite, combining QUIC send and receive streams.
/// 双向流，实现 AsyncRead 和 AsyncWrite，结合了 QUIC 发送和接收流。
pub struct DuplexStream {
  pub(crate) send: quinn::SendStream,
  pub(crate) recv: quinn::RecvStream,
}

impl AsyncRead for DuplexStream {
  fn poll_read(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &mut ReadBuf<'_>,
  ) -> Poll<io::Result<()>> {
    Pin::new(&mut self.recv).poll_read(cx, buf)
  }
}

impl AsyncWrite for DuplexStream {
  fn poll_write(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &[u8],
  ) -> Poll<io::Result<usize>> {
    Pin::new(&mut self.send)
      .poll_write(cx, buf)
      .map_err(io::Error::other)
  }

  fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
    Pin::new(&mut self.send).poll_flush(cx)
  }

  fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
    Pin::new(&mut self.send).poll_shutdown(cx)
  }
}
