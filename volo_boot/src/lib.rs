use std::net::SocketAddr;

use aok::{OK, Void};
use volo::Service;
use volo_grpc::{
  RecvEntryMessage, Request, Response, SendEntryMessage, Status,
  codec::compression::{CompressionEncoding::Zstd, ZstdConfig},
  context::ServerContext,
  server::{NamedService, Server, ServiceBuilder},
};

genv::s!(GRPC_ADDR:String | "0.0.0.0:3333".into());

pub async fn run<S, T, U>(api_server: S) -> Void
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
  log_init::init();
  static_::init().await?;
  let grpc_addr: SocketAddr = GRPC_ADDR.parse().unwrap();
  let grpc_addr = volo::net::Address::from(grpc_addr);

  log::info!("GRPC LISTEN ON {}", grpc_addr);

  let srv = ServiceBuilder::new(api_server)
    .send_compressions(vec![Zstd(Some(ZstdConfig::default()))])
    .accept_compressions(vec![Zstd(None)])
    .build();

  xerr::log!(
    Server::new()
      .add_service(srv)
      .layer_front(volo_layer::Log)
      .run(grpc_addr)
      .await
  );
  OK
}
