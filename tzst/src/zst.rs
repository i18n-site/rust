use std::io::{self, Write};

use zstd::stream::{Decoder, Encoder};

pub fn e(data: impl AsRef<[u8]>) -> io::Result<Vec<u8>> {
  let data = data.as_ref();
  let mut encoder = Encoder::new(Vec::new(), 19)?;
  encoder.write_all(data)?;
  let compressed_data = encoder.finish()?;
  Ok(compressed_data)
}

pub fn d(data: impl AsRef<[u8]>) -> io::Result<Vec<u8>> {
  let data = data.as_ref();
  let mut decoder = Decoder::new(data)?;
  let mut decompressed_data = Vec::new();
  io::copy(&mut decoder, &mut decompressed_data)?;
  Ok(decompressed_data)
}
