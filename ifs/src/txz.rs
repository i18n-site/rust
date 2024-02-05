use std::io::BufReader;

use aok::{Result, OK};
use tokio::task::spawn_blocking;

use crate::pipe::channel;

pub fn d(path: impl AsRef<std::path::Path>, to: impl AsRef<std::path::Path>) -> Result<()> {
  let (mut w, r) = channel(512);
  let mut f = BufReader::new(std::fs::File::open(path)?);
  spawn_blocking(move || xerr::log!(lzma_rs::xz_decompress(&mut f, &mut w)));
  tar::Archive::new(r).unpack(to)?;
  OK
}

// lzma_rs::xz_decompress(&mut r, &mut w)?;
// let tar = w.get_mut();
// tar.seek(std::io::SeekFrom::Start(0))?;
// let tar = tar::Archive::new(tar);
