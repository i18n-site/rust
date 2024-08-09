[‼️]: ✏️README.mdt

# tzst

```rust
use std::{io, path::PathBuf};

use gxhash::gxhash128;
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn main() -> io::Result<()> {
  let root_dir = env!("CARGO_MANIFEST_DIR");
  let root_dir: PathBuf = root_dir.into();
  let relative_paths = vec!["Cargo.toml", "src/lib.rs"];

  let mut w = tzst::W::new();

  w.add_path_li(&root_dir, relative_paths)?;
  w.add_bin("a", b"test")?;

  let compressed_data = w.end()?;
  let hash = gxhash128(&compressed_data, 0);
  info!("{}", hash);

  let t = tzst::r(&compressed_data)?;

  for i in t {
    info!("{:?}", i);
  }

  Ok(())
}
```
