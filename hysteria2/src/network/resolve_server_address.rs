use crate::HysteriaError;

/// Resolve server address.
/// 解析服务器地址。
pub(crate) async fn resolve_server_address(
  server_addr: &str,
) -> Result<std::net::SocketAddr, HysteriaError> {
  tokio::net::lookup_host(server_addr)
    .await?
    .next()
    .ok_or_else(|| HysteriaError::InvalidAddress(server_addr.to_string()))
}
