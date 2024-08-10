[‼️]: ✏️README.mdt

# s3_put

backblaze 必须用 " 应用程序密钥 " 而不是 " 主密钥 ", 不然[会报错 "Malformed Access Key Id"](https://github.com/timmyomahony/craft-remote-backup/issues/11#issuecomment-657661478)

```rust
#![feature(async_closure)]
#![feature(let_chains)]

use std::sync::Arc;

use aok::{Result, OK};
use map_await::{MapAwait, StreamExt};
use s3_put::S3Bucket;
use static_init::constructor;
use tracing::info;
use walkdir::WalkDir;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  let dir = env!("CARGO_MANIFEST_DIR");
  let toml = format!("{dir}/Cargo.toml");
  info!("{}", toml);
  let s3 = Arc::new(S3Bucket::from_env("i18ntmp"));
  let mut iter = WalkDir::new(dir).map_unordered(3, move |i| {
    let s3 = s3.clone();
    async move {
      if let Ok::<walkdir::DirEntry, _>(i) = i {
        let file_type = i.file_type();
        if file_type.is_file() {
          let path = i.path();

          if let Some(url) = path.strip_prefix(dir)?.as_os_str().to_str() {
            info!("begin upload {url}");
            s3.put(url, "text/js", path).await?;
            info!("uploaded {url}");
          }
        }
      }
      OK
    }
  });
  while let Some(r) = iter.next().await {
    r?;
  }
  OK
}
```
