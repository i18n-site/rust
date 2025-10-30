use thiserror::Error;

#[derive(Debug, Error)]
pub enum HysteriaError {
  #[error("无效地址: {0}")]
  InvalidAddress(String),
  #[error("QUIC 连接错误: {0}")]
  QuicConnectionError(#[from] quinn::ConnectionError),
  #[error("QUIC 连接错误: {0}")]
  QuicConnectError(#[from] quinn::ConnectError),
  #[error("I/O 错误: {0}")]
  IoError(#[from] std::io::Error),
  #[error("URL 解析错误: {0}")]
  UrlParseError(#[from] url::ParseError),
  #[error("地址解析错误: {0}")]
  AddressParseError(#[from] std::net::AddrParseError),
  #[error("认证失败")]
  AuthFailed,
  #[error("服务器不支持UDP")]
  UdpNotSupported,
  #[error("QUIC 写入错误: {0}")]
  QuicWriteError(#[from] quinn::WriteError),
  #[error("QUIC 流已关闭")]
  QuicStreamClosed(#[from] quinn::ClosedStream),
  #[error("H3 连接错误: {0}")]
  H3ConnectionError(#[from] h3::error::ConnectionError),
  #[error("H3 流错误: {0}")]
  H3StreamError(#[from] h3::error::StreamError),
  #[error("TCP 连接错误: {0}")]
  TcpConnectError(String),
}
