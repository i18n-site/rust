use bytes::Bytes;
use tonic::transport::Endpoint;

pub fn endpoint(addr: impl Into<Bytes>) -> Result<Endpoint, tonic::transport::Error> {
  use tonic::transport::Endpoint;
  Ok(
    Endpoint::from_shared(addr)?
      .http2_keep_alive_interval(std::time::Duration::from_secs(60))
      .keep_alive_while_idle(true),
  )
}
