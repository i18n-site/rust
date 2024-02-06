use std::{
  hash::Hasher,
  io::{Read, Write},
};

use aok::Result;
pub use lzma_rs as lzma;
pub use tar::{self, Archive as Tar};

use crate::hash;

pub fn d<H: Hasher + Default>(reader: impl Read, to: impl AsRef<std::path::Path>) -> Result<H> {
  let (mut read, mut write) = pipe::pipe();

  let hasher = hash!(: H, reader, write);
  write.flush()?;

  let (tar_read, mut tar_write) = pipe::pipe();
  let _ = std::thread::spawn(move || xerr::log!(lzma::xz_decompress(&mut read, &mut tar_write)));
  Tar::new(tar_read).unpack(to)?;
  Ok(hasher)
}
