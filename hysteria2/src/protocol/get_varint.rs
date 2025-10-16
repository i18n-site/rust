use std::io;

use tokio::io::AsyncReadExt;

// Helper function to read a QUIC variable-length integer.
// 辅助函数，用于读取QUIC可变长度整数。
pub(crate) async fn get_varint<R: tokio::io::AsyncRead + Unpin>(reader: &mut R) -> io::Result<u64> {
  let first_byte = reader.read_u8().await?;
  let tag = first_byte >> 6;
  let val = match tag {
    0 => u64::from(first_byte & 0x3F),
    1 => {
      let second_byte = reader.read_u8().await?;
      u64::from(u16::from_be_bytes([first_byte, second_byte]) & 0x3FFF)
    }
    2 => {
      let mut bytes = [0u8; 4];
      bytes[0] = first_byte;
      reader.read_exact(&mut bytes[1..]).await?;
      u64::from(u32::from_be_bytes(bytes) & 0x3FFFFFFF)
    }
    3 => {
      let mut bytes = [0u8; 8];
      bytes[0] = first_byte;
      reader.read_exact(&mut bytes[1..]).await?;
      u64::from_be_bytes(bytes) & 0x3FFFFFFFFFFFFFFF
    }
    _ => unreachable!(),
  };
  Ok(val)
}
