[‼️]: ✏️README.mdt

# ifs

```rust
use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  ifs::txz::d(
    "/Users/z/Downloads/0.1.70.aarch64-pc-windows-msvc.tar.xz",
    "/tmp/t/x/y",
  )?;
  OK
}
```
