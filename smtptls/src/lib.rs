use std::net::IpAddr;

use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
use thiserror::Error;
use tokio::{
  io::{AsyncReadExt, AsyncWriteExt},
  net::TcpStream,
  time::timeout,
};
use tokio_rustls::TlsConnector;

// SMTP 状态码常量
const SMTP_READY: &str = "220";
const SMTP_AUTH_CONTINUE: &str = "334";
const SMTP_AUTH_SUCCESS: &str = "235";

mod cert_verify;
pub use cert_verify::cert_verify;

#[derive(Error, Debug)]
pub enum SmtpTlsError {
  #[error("连接超时")]
  Timeout,
  #[error("TLS错误: {0}")]
  Tls(#[from] rustls::Error),
  #[error("IO错误: {0}")]
  Io(#[from] std::io::Error),
  #[error("认证失败: {0}")]
  AuthFailed(String),
  #[error("无效的服务器响应")]
  InvalidResponse,
  #[error("证书域名不匹配: 期望 {expected}, 实际 {actual}")]
  CertDomainMismatch { expected: String, actual: String },
  #[error("{host} {addr} 邮箱证书已经过期")]
  CertExpired { host: String, addr: String },
  #[error(" {host} {addr} 邮箱证书还有 {remain_days} 天, 即将过期")]
  CertExpiring {
    host: String,
    addr: String,
    remain_days: u32,
  },
}

async fn read<T: AsyncReadExt + std::marker::Unpin>(
  stream: &mut T,
) -> Result<String, std::io::Error> {
  let mut buf = [0; 1024];
  let n = stream.read(&mut buf).await?;
  Ok(String::from_utf8_lossy(&buf[..n]).to_string())
}

pub async fn smtptls(
  host: &str,
  addr: (IpAddr, u16),
  user: &str,
  password: &str,
  timeout_secs: u64,
) -> Result<u64, SmtpTlsError> {
  let timeout_duration = std::time::Duration::from_secs(timeout_secs);

  let mut stream = timeout(timeout_duration, TcpStream::connect(addr))
    .await
    .map_err(|_| SmtpTlsError::Timeout)??;

  let response = read(&mut stream).await?;
  if !response.starts_with(SMTP_READY) {
    return Err(SmtpTlsError::InvalidResponse);
  }

  stream
    .write_all(format!("EHLO {host}\r\n").as_bytes())
    .await?;

  let _ = read(&mut stream).await?;

  stream.write_all(b"STARTTLS\r\n").await?;

  let _ = read(&mut stream).await?;

  if !response.starts_with(format!("{SMTP_READY} ").as_str()) {
    return Err(SmtpTlsError::InvalidResponse);
  }

  let config = tlsinit::CLIENT.clone();
  let connector = TlsConnector::from(config);

  let mut tls_stream = timeout(
    timeout_duration,
    connector.connect(
      rustls::pki_types::ServerName::try_from(host.to_owned())
        .map_err(|_| SmtpTlsError::InvalidResponse)?,
      stream,
    ),
  )
  .await
  .map_err(|_| SmtpTlsError::Timeout)??;

  tls_stream
    .write_all(format!("EHLO {host}\r\n").as_bytes())
    .await?;

  let _ = read(&mut tls_stream).await?;

  tls_stream.write_all(b"AUTH PLAIN\r\n").await?;

  let response = read(&mut tls_stream).await?;

  if !response.starts_with(format!("{SMTP_AUTH_CONTINUE} ").as_str()) {
    return Err(SmtpTlsError::AuthFailed(format!(
      "AUTH PLAIN → 服务器响应：{}",
      response.trim()
    )));
  }

  // 修改认证格式为 \0username\0password
  let auth_string = format!("\0{user}\0{password}");
  let encoded_auth = format!("{}\r\n", STANDARD.encode(auth_string));
  tls_stream.write_all(encoded_auth.as_bytes()).await?;

  let response = read(&mut tls_stream).await?;

  if !response.starts_with(format!("{SMTP_AUTH_SUCCESS} ").as_str()) {
    return Err(SmtpTlsError::AuthFailed(format!(
      "认证失败，服务器响应：{}",
      response.trim()
    )));
  }

  cert_verify(&tls_stream, host, format!("{}:{}", addr.0, addr.1)).await
}
