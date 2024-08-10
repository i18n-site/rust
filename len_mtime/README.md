[‼️]: ✏️README.mdt

# len_mtime

len mtime 用 db 存

语言文件存

lang hash_len src_hash

```rust
use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

// #[tokio::test]
// async fn test() -> Result<()> {
//   info!("{}", 123456);
//   OK
// }

#[test]
fn test() -> Result<()> {
  let workdir = env!("CARGO_MANIFEST_DIR");
  let workdir: std::path::PathBuf = workdir.into();
  let dbdir = workdir.join("tests/db");
  let mut len_mtime = len_mtime::LenMtime::load(&dbdir, &workdir)?;

  for i in len_mtime.is_change([
    ("Cargo.toml", vec![]),
    // "README.mdt",
    // "src/lib.rs",
    // "tests/main.rs",
  ])? {
    dbg!(i);
  }

  let need_save = ["Cargo.toml"];
  len_mtime.save(need_save)?;
  OK
}
```
