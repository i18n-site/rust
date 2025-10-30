use std::io;

use tokio::io::{AsyncReadExt, sink};

use super::{get_varint::get_varint, tcp_response_status::TCPResponseStatus};

/// 从服务器解析TCP响应
pub async fn read_tcp_response<R: tokio::io::AsyncRead + Unpin>(
  reader: &mut R,
) -> io::Result<(TCPResponseStatus, String)> {
  let status_byte = reader.read_u8().await?;
  let status = if status_byte == 0x00 {
    TCPResponseStatus::Ok
  } else {
    TCPResponseStatus::Error
  };

  let msg_len = get_varint(reader).await? as usize;
  let mut msg_buf = vec![0u8; msg_len];
  reader.read_exact(&mut msg_buf).await?;
  let message = String::from_utf8_lossy(&msg_buf).to_string();

  // Ignore padding.
  // 忽略填充。
  let padding_len = get_varint(reader).await?;
  if padding_len > 0 {
    // Efficiently discard padding without allocating a buffer.
    // 高效地丢弃填充，无需分配缓冲区。
    tokio::io::copy(&mut reader.take(padding_len), &mut sink()).await?;
  }

  Ok((status, message))
}
