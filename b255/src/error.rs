use thiserror::Error;

/// 解码时可能发生的错误。
#[derive(Error, Debug, PartialEq, Eq)]
pub enum DecodeError {
  /// 表示在输入中遇到了无效的字节。
  /// Base255 编码后的数据不应包含冒号 (':')。
  #[error("无效字节: {0}")]
  InvalidByte(u8),
}
