use std::borrow::Borrow;

use num_traits::ops::bytes::ToBytes;

pub fn bin_u64(bin: impl AsRef<[u8]>) -> u64 {
  let bin = bin.as_ref();
  let mut b = [0u8; 8];
  b[..bin.len()].copy_from_slice(bin);
  u64::from_le_bytes(b)
}

pub fn bin_u16(bin: impl AsRef<[u8]>) -> u16 {
  let bin = bin.as_ref();
  let mut b = [0u8; 2];
  b[..bin.len()].copy_from_slice(bin);
  u16::from_le_bytes(b)
}

pub fn to_bin(n: impl ToBytes) -> Box<[u8]> {
  let n = n.to_le_bytes();
  let n = n.borrow();
  let mut i = n.len();
  while i > 0 {
    let p = i - 1;
    if n[p] != 0 {
      break;
    }
    i = p;
  }
  Box::from(&n[..i])
}

pub fn u8_bin(n: u8) -> Box<[u8]> {
  if n == 0 {
    return [].into();
  }
  [n].into()
}

pub fn bin_u8(bin: impl AsRef<[u8]>) -> u8 {
  let bin = bin.as_ref();
  if bin.is_empty() {
    return 0;
  };
  bin[0]
}
