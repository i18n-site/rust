[‼️]: ✏️README.mdt

# minjs

```rust
use std::path::PathBuf;

use aok::{Result, OK};
use minjs::minjs;
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let dir = env!("CARGO_MANIFEST_DIR");
  let dir: PathBuf = dir.into();

  let r = minjs(dir.join("tests/test.js"))?;
  dbg!(r);
  OK
}
```
