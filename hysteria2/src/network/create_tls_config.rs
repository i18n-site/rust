use std::sync::Arc;

use super::insecure_verifier::InsecureVerifier;
use crate::HysteriaError;

const ALPN_H3: &[u8] = b"h3";

/// Create TLS configuration.
/// 创建TLS配置。
pub(crate) fn create_tls_config(
  insecure: bool,
) -> Result<Arc<rustls::ClientConfig>, HysteriaError> {
  let builder = rustls::ClientConfig::builder();

  let builder_with_certs = if insecure {
    builder
      .dangerous()
      .with_custom_certificate_verifier(Arc::new(InsecureVerifier))
  } else {
    let mut roots = rustls::RootCertStore::empty();
    roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    builder.with_root_certificates(roots)
  };

  let mut crypto = builder_with_certs.with_no_client_auth();
  crypto.alpn_protocols = vec![ALPN_H3.to_vec()];
  Ok(Arc::new(crypto))
}
