use super::duplex_stream::DuplexStream;
use crate::{
  HysteriaError,
  protocol::{TCPResponseStatus, read_tcp_response, tcp_request},
};

/// Hysteria client, which holds a QUIC connection and provides methods for interacting with the proxy.
/// Hysteria 客户端，持有 QUIC 连接并提供与代理交互的方法。
pub struct HysteriaClient {
  pub(crate) quic_connection: quinn::Connection,
}

impl HysteriaClient {
  /// Establish a proxied TCP connection to the given address.
  /// 建立到给定地址的代理 TCP 连接。
  pub async fn tcp_connect(&self, address: impl AsRef<str>) -> Result<DuplexStream, HysteriaError> {
    let (mut send, mut recv) = self.quic_connection.open_bi().await?;

    send.write_all(&tcp_request(address, 0)).await?;
    // Do not close the send stream - we need it for the duplex connection.
    // 不结束发送流 - 我们需要它用于双向连接。

    // Wait for server response.
    // 等待服务器响应。
    let (status, msg) = read_tcp_response(&mut recv).await?;
    if status != TCPResponseStatus::Ok {
      return Err(HysteriaError::TcpConnectError(msg));
    }

    Ok(DuplexStream { send, recv })
  }
}
