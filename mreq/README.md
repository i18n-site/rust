[‼️]: ✏️README.mdt

# mreq

```rust
use aok::{Result, OK};
use mreq::Mreq;
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  let mut req = Mreq::new(
    [
      // "httpstat.us",
      "jsd.onmicrosoft.cn",
      "cdn.jsdelivr.net",
      "cdn.jsdmirror.com",
    ],
    [],
  );
  let v = req.get("npm/i18md/.v").await?;
  let v = String::from_utf8_lossy(&v);
  info!("1 {}", v);
  let v = req.get("npm/i18md/.v").await?;
  let v = String::from_utf8_lossy(&v);
  info!("2 {}", v);
  // let v = req.get("npm/i18md/.v").await?;
  // let v = String::from_utf8_lossy(&v);
  // info!("{}", v);
  OK
}
```
