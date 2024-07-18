use std::io::{Read, Write};

use aok::Result;
pub use digest::Digest;
pub use lzma_rs as lzma;
pub use tar::{self, Archive as Tar};

use crate::hash;

pub fn d<H: Digest>(
  reader: impl Read,
  to: impl AsRef<std::path::Path> + Send + 'static,
) -> Result<H> {
  let (mut read, mut write) = pipe::pipe();
  let (tar_read, mut tar_write) = pipe::pipe();
  let unxz = std::thread::spawn(move || xerr::log!(lzma::xz_decompress(&mut read, &mut tar_write)));
  let untar = std::thread::spawn(move || xerr::log!(Tar::new(tar_read).unpack(to)));
  let hasher = hash!(: H, reader, write);
  write.flush()?;

  drop(write); // 不然 xz 不会结束
  let _ = unxz.join();
  let _ = untar.join();

  Ok(hasher.0)
}
