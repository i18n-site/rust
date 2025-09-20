use std::{
  future::Future,
  io,
  pin::Pin,
  sync::Arc,
  task::{self, Poll},
};

use hyper::Uri;
use hyper_util::client::legacy::connect::Connection;
use pin_project::pin_project;
use shadowsocks::{
  ProxyClientStream,
  config::{ServerConfig, ServerType},
  context::{Context, SharedContext},
  net::TcpStream,
  relay::Address,
};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tower::Service;

use self::error::SsConnectorError;

pub mod error;
#[cfg(feature = "reqwest")]
pub mod reqwest;

#[cfg(feature = "reqwest")]
pub use reqwest::SsMiddleware;

#[derive(Clone)]
pub struct SsConnector {
  svr_cfg: Arc<ServerConfig>,
  context: SharedContext,
}

impl SsConnector {
  pub fn new(url: &str) -> Result<Self, SsConnectorError> {
    let svr_cfg = ServerConfig::from_url(url)?;
    let context = Context::new(ServerType::Local);
    Ok(Self {
      svr_cfg: Arc::new(svr_cfg),
      context: Arc::new(context),
    })
  }
}

fn uri_to_address(uri: &Uri) -> Result<Address, io::Error> {
  let host = uri
    .host()
    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "URI MISS HOST"))?;
  let port = uri.port_u16().unwrap_or_else(|| match uri.scheme_str() {
    Some("https") => 443,
    _ => 80,
  });
  Ok(Address::DomainNameAddress(host.to_string(), port))
}

#[pin_project]
pub struct SsStream(#[pin] ProxyClientStream<TcpStream>);

impl hyper::rt::Read for SsStream {
  fn poll_read(
    self: Pin<&mut Self>,
    cx: &mut task::Context<'_>,
    mut buf: hyper::rt::ReadBufCursor<'_>,
  ) -> Poll<Result<(), std::io::Error>> {
    let mut tbuf = ReadBuf::uninit(unsafe { buf.as_mut() });
    match self.project().0.poll_read(cx, &mut tbuf) {
      Poll::Ready(Ok(())) => {
        let n = tbuf.filled().len();
        unsafe {
          buf.advance(n);
        }
        Poll::Ready(Ok(()))
      }
      Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
      Poll::Pending => Poll::Pending,
    }
  }
}

impl hyper::rt::Write for SsStream {
  fn poll_write(
    self: Pin<&mut Self>,
    cx: &mut task::Context<'_>,
    buf: &[u8],
  ) -> Poll<Result<usize, std::io::Error>> {
    self.project().0.poll_write(cx, buf)
  }

  fn poll_flush(
    self: Pin<&mut Self>,
    cx: &mut task::Context<'_>,
  ) -> Poll<Result<(), std::io::Error>> {
    self.project().0.poll_flush(cx)
  }

  fn poll_shutdown(
    self: Pin<&mut Self>,
    cx: &mut task::Context<'_>,
  ) -> Poll<Result<(), std::io::Error>> {
    self.project().0.poll_shutdown(cx)
  }
}

impl Connection for SsStream {
  fn connected(&self) -> hyper_util::client::legacy::connect::Connected {
    hyper_util::client::legacy::connect::Connected::new()
  }
}

impl Service<Uri> for SsConnector {
  type Response = SsStream;
  type Error = io::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  fn poll_ready(&mut self, _cx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, uri: Uri) -> Self::Future {
    let svr_cfg = self.svr_cfg.clone();
    let context = self.context.clone();

    Box::pin(async move {
      let addr = uri_to_address(&uri)?;
      let stream = ProxyClientStream::connect(context, &svr_cfg, &addr).await?;
      Ok(SsStream(stream))
    })
  }
}
