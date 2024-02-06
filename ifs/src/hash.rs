use std::path::Path;

use blake3::Hasher;
use tokio::{
  fs::File,
  io::{AsyncReadExt, BufReader},
};

pub async fn hash(path: impl AsRef<Path>) -> Result<[u8; 32], std::io::Error> {
  let mut hasher = Hasher::new();
  let mut file = BufReader::new(File::open(path).await?);

  let mut buf = [0; 65536];
  loop {
    let n = file.read(&mut buf).await?;
    if n == 0 {
      break;
    }
    hasher.update(&buf[..n]);
  }
  Ok(*hasher.finalize().as_bytes())
}
