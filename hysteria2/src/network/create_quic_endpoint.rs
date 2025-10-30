use std::{io, sync::Arc};

use quinn::{ClientConfig, Endpoint};

use crate::HysteriaError;

/// Create a QUIC endpoint.
/// 创建QUIC端点。
pub(crate) fn create_quic_endpoint(
  client_crypto: Arc<rustls::ClientConfig>,
) -> Result<Endpoint, HysteriaError> {
  let client_config = ClientConfig::new(Arc::new(
    quinn::crypto::rustls::QuicClientConfig::try_from(client_crypto)
      .map_err(|e| HysteriaError::IoError(io::Error::new(io::ErrorKind::InvalidInput, e)))?,
  ));

  let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
  endpoint.set_default_client_config(client_config);
  Ok(endpoint)
}
