use bytes::BufMut;

// Helper function to write a QUIC variable-length integer - optimized with match for performance.
// 辅助函数，用于写入QUIC可变长度整数 - 使用match优化以提高性能。
pub(crate) fn put_varint(val: u64, buf: &mut bytes::BytesMut) {
  match val {
    0..=63 => buf.put_u8(val as u8),
    64..=16383 => buf.put_u16(((val & 0x3FFF) | 0x4000) as u16),
    16384..=1073741823 => buf.put_u32(((val & 0x3FFFFFFF) | 0x80000000) as u32),
    _ => buf.put_u64((val & 0x3FFFFFFFFFFFFFFF) | 0xC000000000000000),
  }
}
