use std::{io::Read, path::Path};

use crate::{HashLen, hash};

/**
这个函数首先使用BLAKE3哈希算法计算给定二进制数据的哈希值，
然后将原始二进制数据的长度（以字节为单位）追加到哈希值的末尾。
最终返回一个包含哈希值和长度的字节切片的Box。
*/
pub fn b3(bin: impl AsRef<[u8]>) -> [u8; 32] {
  let bin = bin.as_ref();
  let mut hasher = blake3::Hasher::new();
  hasher.update(bin);
  *hasher.finalize().as_bytes()
}

pub fn b3_len(bin: impl AsRef<[u8]>) -> Box<[u8]> {
  let bin = bin.as_ref();
  [&b3(bin)[..], &intbin::to_bin(bin.len() as u64)[..]]
    .concat()
    .into()
}

pub fn b3_len_fp(path: impl AsRef<Path>) -> Result<HashLen, std::io::Error> {
  let file = std::fs::File::open(path)?;
  let (hash, len) = hash!(: blake3::Hasher, file);
  Ok(HashLen {
    len,
    hash: [hash.finalize().as_bytes(), &intbin::to_bin(len as u64)[..]].concat(),
  })
}
