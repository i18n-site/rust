use crate::HysteriaError;

const HYSTERIA_AUTH_STATUS: u16 = 233;

/// 验证认证响应
pub(crate) fn validate_auth_response(resp: &http::Response<()>) -> Result<(), HysteriaError> {
  if resp.status() != HYSTERIA_AUTH_STATUS {
    return Err(HysteriaError::AuthFailed);
  }

  // Check for UDP support (optional).
  // 检查UDP支持（可选）。
  if resp
    .headers()
    .get("Hysteria-UDP")
    .and_then(|v| v.to_str().ok())
    != Some("true")
  {
    tracing::warn!("Server does not explicitly support UDP relay.");
  }

  Ok(())
}
