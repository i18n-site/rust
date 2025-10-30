use anyhow::Result;
use tokio::net::TcpStream;
use x509_parser::prelude::*;

use crate::SmtpTlsError;

pub async fn cert_verify(
  tls_stream: &tokio_rustls::client::TlsStream<TcpStream>,
  host: &str,
  addr: String,
) -> Result<u64, SmtpTlsError> {
  let (_, server_conn) = tls_stream.get_ref();
  let cert = server_conn
    .peer_certificates()
    .and_then(|certs| certs.first())
    .ok_or(SmtpTlsError::InvalidResponse)?;

  let (_, cert) =
    X509Certificate::from_der(cert.as_ref()).map_err(|_| SmtpTlsError::InvalidResponse)?;

  let cert_names = cert
    .subject_alternative_name()
    .map_err(|_| SmtpTlsError::InvalidResponse)?
    .ok_or(SmtpTlsError::InvalidResponse)?;

  let found_domain = cert_names
    .value
    .general_names
    .iter()
    .filter_map(|name| {
      if let x509_parser::extensions::GeneralName::DNSName(domain) = name {
        Some(domain.to_string())
      } else {
        None
      }
    })
    .find(|domain| domain == host || (domain.starts_with("*.") && host.ends_with(&domain[2..])));

  if let Some(domain) = found_domain {
    if domain != host && !domain.starts_with("*.") {
      return Err(SmtpTlsError::CertDomainMismatch {
        expected: host.to_string(),
        actual: domain,
      });
    }
  } else {
    return Err(SmtpTlsError::CertDomainMismatch {
      expected: host.to_string(),
      actual: "未找到匹配的域名".to_string(),
    });
  }

  let not_after = cert.validity().not_after.timestamp() as u64;
  let now = sts::sec();

  if now >= not_after {
    return Err(SmtpTlsError::CertExpired {
      host: host.into(),
      addr,
    });
  }

  let remain_sec = not_after - now;
  let remain_days = remain_sec / 86400;
  if remain_days <= 9 {
    return Err(SmtpTlsError::CertExpiring {
      host: host.into(),
      addr,
      remain_days: remain_days as u32,
    });
  }
  Ok(remain_sec)
}
