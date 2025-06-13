use std::{hash::Hasher, io::Read, path::Path};

use crate::{hash_len_concat, HASH128_LEN};

#[derive(Debug, Clone)]
pub struct HashLen {
  pub hash: Vec<u8>,
  pub len: usize,
}
/*
Hasher 实现 std::io::Write ，因此可以使用 std::io::copy 它从任何读取器更新 A Hasher 。

不幸的是，这种标准方法可能会限制性能，因为 copy 目前使用的内部 8 KiB 缓冲区不够大，无法利用所有 SIMD 指令集。

特别是，AVX-512 需要 16 KiB 缓冲区

https://docs.rs/blake3/latest/blake3/struct.Hasher.html
*/

pub const BUFFER_SIZE: usize = 16384;

pub fn hash_len(path: impl AsRef<Path>) -> Result<HashLen, std::io::Error> {
  let mut reader = std::fs::File::open(path)?;
  let mut hasher = crate::hasher();
  let mut buf = [0; BUFFER_SIZE];
  let mut len = reader.read(&mut buf)?;

  let hash = if len > HASH128_LEN {
    hasher.write(&buf[..len]);
    loop {
      let n = reader.read(&mut buf)?;
      if n == 0 {
        break;
      }
      hasher.write(&buf[..n]);
      len += n;
    }
    hash_len_concat(hasher.digest128(), len)
  } else {
    buf[..len].into()
  };

  Ok(HashLen { hash, len })
}
