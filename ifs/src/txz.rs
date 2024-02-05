use std::io::BufReader;

use aok::{Result, OK};

use crate::pipe::channel;

pub fn d(path: impl AsRef<std::path::Path>, to: impl AsRef<std::path::Path>) -> Result<()> {
  let (mut w, r) = channel(512);
  let mut f = BufReader::new(std::fs::File::open(path)?);
  let _ = std::thread::spawn(move || xerr::log!(lzma_rs::xz_decompress(&mut f, &mut w)));
  tar::Archive::new(r).unpack(to)?;
  OK
}
