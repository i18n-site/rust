use std::borrow::Borrow;

use num_traits::ops::bytes::ToBytes;

/// 内部泛型辅助函数：将字节切片安全且无越界检查地转换为固定长度数组
#[inline]
fn bin_to_int<const N: usize>(bin: &[u8]) -> [u8; N] {
  let mut b = [0u8; N];
  let len = bin.len().min(N);
  // SAFETY: `len` 不会超过 `N`（由 `.min(N)` 保证），也不会超过 `bin.len()`，
  // 因此 `..len` 范围对 `b` 和 `bin` 均 100% 安全。
  unsafe {
    b.get_unchecked_mut(..len)
      .copy_from_slice(bin.get_unchecked(..len));
  }
  b
}

pub fn bin_u64(bin: impl AsRef<[u8]>) -> u64 {
  u64::from_le_bytes(bin_to_int(bin.as_ref()))
}

pub fn bin_u16(bin: impl AsRef<[u8]>) -> u16 {
  u16::from_le_bytes(bin_to_int(bin.as_ref()))
}

pub fn to_bin(n: impl ToBytes) -> Box<[u8]> {
  let n = n.to_le_bytes();
  let bytes = n.borrow();
  let len = bytes.iter().rposition(|&x| x != 0).map_or(0, |i| i + 1);
  // SAFETY: `len` 不可能超过 `bytes.len()`，因此 `..len` 是安全的
  let slice = unsafe { bytes.get_unchecked(..len) };
  Box::from(slice)
}

pub fn u8_bin(n: u8) -> Box<[u8]> {
  if n == 0 {
    Box::default()
  } else {
    Box::from([n])
  }
}

pub fn bin_u8(bin: impl AsRef<[u8]>) -> u8 {
  bin.as_ref().first().copied().unwrap_or(0)
}
