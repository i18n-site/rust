// Constant definition to avoid magic numbers.
// 常量定义，避免魔法数字。
const PADDING_MIN_LEN: u8 = 8;
const PADDING_MAX_EXTRA: u8 = 32;

// Pre-defined character set to avoid recreation on each generation.
// 预定义的字符集，避免每次生成时重新创建。
const PADDING_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// Generate random padding - optimized version.
/// 生成随机填充 - 优化版本。
pub(crate) fn generate_padding() -> String {
  use rand::Rng;
  let mut rng = rand::rng();
  let padding_len = rng.random_range(PADDING_MIN_LEN..PADDING_MIN_LEN + PADDING_MAX_EXTRA);
  let mut padding = String::with_capacity(padding_len as usize);

  for _ in 0..padding_len {
    let idx = rng.random_range(0..PADDING_CHARS.len());
    padding.push(PADDING_CHARS[idx] as char);
  }

  padding
}
