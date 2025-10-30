use super::{
  authenticate_connection::authenticate_connection, create_quic_endpoint::create_quic_endpoint,
  create_tls_config::create_tls_config, hysteria_client::HysteriaClient,
  port_hopping::try_port_hopping_connection, resolve_server_address::resolve_server_address,
};
use crate::{Result, config::Config};

/// Connect to the Hysteria server and perform the authentication handshake.
/// 连接到 Hysteria 服务器并执行认证握手。
pub async fn connect(config: &Config) -> Result<HysteriaClient> {
  let server_addr = resolve_server_address(&config.server_addr).await?;
  let server_ip = server_addr.ip();

  let client_crypto = create_tls_config(config.insecure)?;
  let endpoint = create_quic_endpoint(client_crypto)?;

  let conn = try_port_hopping_connection(&endpoint, config, server_addr, server_ip).await?;

  authenticate_connection(&conn, config).await?;

  Ok(HysteriaClient {
    quic_connection: conn,
  })
}
