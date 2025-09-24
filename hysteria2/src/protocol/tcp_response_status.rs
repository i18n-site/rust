/// Hysteria 2 TCP response message status.
/// Hysteria 2 TCP 响应消息状态。
#[derive(Debug, PartialEq, Eq)]
pub enum TCPResponseStatus {
  Ok,
  Error,
}
