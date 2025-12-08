use anyhow::Result;
use http_grpc::http_grpc;
use volo_http::{
  Address, Server,
  server::route::{Router, post},
};

use crate::env_addr;

pub async fn http<T: http_grpc::Grpc + 'static>(init: impl Fn(Router) -> Router) -> Result<()> {
  let app = Router::new().route("/", post(http_grpc::<T>));
  let addr: Address = env_addr("HTTP", 3334).into();
  Server::new(init(app))
    .run(addr)
    .await
    .map_err(|e| anyhow::anyhow!(e))?;
  Ok(())
}
