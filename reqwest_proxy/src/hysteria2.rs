use std::{
  io,
  sync::Arc,
  task::{self, Poll},
};

use ::hysteria2::{DuplexStream, config::Config, connect};
use hyper::Uri;
use hyper_util::rt::TokioIo;
use tower::Service;

use crate::conn::hysteria2::ConnFuture;
use crate::{middleware::ProxyMiddleware, stream::Stream};

pub const SCHEME: &str = "hysteria2";
pub type StreamType = Stream<DuplexStream>;
pub type StreamEnumType = StreamType;

#[derive(Clone)]
pub struct Conn {
  conf: Arc<Config>,
}

impl Conn {
  pub fn new(url: &str) -> crate::Result<Self> {
    Ok(Self {
      conf: Arc::new(Config::from_url(url)?),
    })
  }
}

impl Service<Uri> for Conn {
  type Response = Stream<DuplexStream>;
  type Error = io::Error;
  type Future = ConnFuture;

  fn poll_ready(&mut self, _cx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, uri: Uri) -> Self::Future {
    let conf = self.conf.clone();
    let fut = Box::pin(async move {
      let (host, port) = uri_to_host_port::parse(&uri).map_err(io::Error::other)?;
      let client = connect(&conf).await.map_err(io::Error::other)?;
      let stream = client
        .tcp_connect(format!("{host}:{port}"))
        .await
        .map_err(io::Error::other)?;
      Ok(Stream(TokioIo::new(stream)))
    });
    ConnFuture { fut }
  }
}

pub fn from_url(url: &str) -> crate::Result<ProxyMiddleware<Conn>> {
  let connector = Conn::new(url)?;
  Ok(ProxyMiddleware::new(connector))
}
