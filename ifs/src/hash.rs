use std::path::Path;

use digest::Digest;
use tokio::{fs::File, io::AsyncReadExt};

#[macro_export]
macro_rules! hash {
  ($($await:ident)? : $Hasher:ty, $reader:expr $(,$write:ident)?) => {{
    let mut hasher = <$Hasher>::new();
    let mut reader = $reader;
/*
Hasher 实现 std::io::Write ，因此可以使用 std::io::copy 它从任何读取器更新 A Hasher 。

不幸的是，这种标准方法可能会限制性能，因为 copy 目前使用的内部 8 KiB 缓冲区不够大，无法利用所有 SIMD 指令集。

特别是，AVX-512 需要 16 KiB 缓冲区

https://docs.rs/blake3/latest/blake3/struct.Hasher.html
*/
    let mut buf = [0; 16384];

    loop {
      let n = reader.read(&mut buf)$(.$await)??;
      if n == 0 {
        break;
      }
      let bin = &buf[..n];
      hasher.update(bin);
      $(let _ = $write.write(bin)?;)?
    }
    hasher
  }};
}

pub async fn hash<H: Digest>(path: impl AsRef<Path>) -> Result<H, std::io::Error> {
  let file = File::open(path).await?;
  Ok(hash!(await: H, file))
}
