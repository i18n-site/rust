use std::{
  io,
  sync::Arc,
  task::{self, Poll},
};

use ::shadowsocks::{
  ProxyClientStream,
  config::{ServerConfig, ServerType},
  context::{Context, SharedContext},
  net::TcpStream,
  relay::Address,
};
use hyper::Uri;
use hyper_util::rt::TokioIo;
use tower::Service;

use crate::conn::shadowsocks::ConnFuture;
use crate::{middleware::ProxyMiddleware, stream::Stream};

pub const SCHEME: &str = "ss";
pub type StreamType = Stream<ProxyClientStream<TcpStream>>;
pub type StreamEnumType = Box<StreamType>;

#[derive(Clone)]
pub struct Conn {
  cfg: Arc<ServerConfig>,
  context: SharedContext,
}

impl Conn {
  pub fn new(url: &str) -> crate::Result<Self> {
    let cfg = ServerConfig::from_url(url)?;
    let context = Context::new(ServerType::Local);
    Ok(Self {
      cfg: Arc::new(cfg),
      context: Arc::new(context),
    })
  }
}

impl Service<Uri> for Conn {
  type Response = StreamType;
  type Error = io::Error;
  type Future = ConnFuture;

  fn poll_ready(&mut self, _cx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, uri: Uri) -> Self::Future {
    let context = self.context.clone();
    let cfg = self.cfg.clone();
    let fut = Box::pin(async move {
      let addr = uri_to_address(&uri)?;
      let stream = ProxyClientStream::connect(context, &cfg, &addr).await?;
      Ok(Stream(TokioIo::new(stream)))
    });
    ConnFuture { fut }
  }
}

fn uri_to_address(uri: &Uri) -> Result<Address, io::Error> {
  let (host, port) =
    uri_to_host_port::parse(uri).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
  Ok(Address::DomainNameAddress(host.to_string(), port))
}

pub fn from_url(url: &str) -> crate::Result<ProxyMiddleware<Conn>> {
  let connector = Conn::new(url)?;
  Ok(ProxyMiddleware::new(connector))
}
