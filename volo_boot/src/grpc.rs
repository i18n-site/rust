use volo::Service;
use volo_grpc::{
  RecvEntryMessage, Request, Response, SendEntryMessage, Status,
  codec::compression::{CompressionEncoding::Zstd, ZstdConfig},
  context::ServerContext,
  server::{NamedService, Server, ServiceBuilder},
};

use crate::{Result, env_addr};

pub async fn grpc<S, T, U>(api_server: S, init: impl Fn(Server) -> Server) -> Result
where
  S: 'static
    + Service<ServerContext, Request<T>, Response = Response<U>>
    + Sync
    + Send
    + NamedService
    + Clone,
  S::Error: Into<Status>,
  T: RecvEntryMessage + 'static,
  U: SendEntryMessage + 'static,
{
  let addr: volo::net::Address = env_addr("GRPC", 3333).into();

  let compression_config = vec![Zstd(Some(ZstdConfig::default()))];
  let srv = ServiceBuilder::new(api_server)
    .send_compressions(compression_config.clone())
    .accept_compressions(compression_config)
    .build();

  init(Server::new())
      .add_service(srv)
      // .layer_front(volo_layer::Log)
      .run(addr)
      .await
}
