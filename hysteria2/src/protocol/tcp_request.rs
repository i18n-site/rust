use bytes::{BufMut, Bytes, BytesMut};
use rand::RngCore;

use super::put_varint::put_varint;

pub fn tcp_request(address: impl AsRef<str>, padding_len: usize) -> Bytes {
  let address = address.as_ref();
  // Pre-calculate buffer size to avoid reallocation.
  // 预计算缓冲区大小以避免重新分配。
  let estimated_size = 8 + address.len() + padding_len + 16; // Leave extra space for varints. / 为varints留出额外空间。
  let mut buf = BytesMut::with_capacity(estimated_size);

  // tcp_request ID.
  // tcp_request ID。
  put_varint(0x401, &mut buf);
  // Address.
  // 地址。
  put_varint(address.len() as u64, &mut buf);
  buf.put(address.as_bytes());
  // Padding.
  // 填充。
  put_varint(padding_len as u64, &mut buf);
  if padding_len > 0 {
    // Generate random padding.
    // 生成随机填充。
    let mut padding = vec![0u8; padding_len];
    let mut rng = rand::rng();
    rng.fill_bytes(&mut padding);
    buf.put(padding.as_slice());
  }
  buf.freeze()
}
