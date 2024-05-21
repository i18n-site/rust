[‼️]: ✏️README.mdt

# bjs

```rust
use std::collections::HashMap;

use aok::{Result, OK};
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let root = std::env!("CARGO_MANIFEST_DIR");

  let root = format!("{root}/tests");
  let ctx = &mut bjs::ctx(&root, &root);
  let mut map1 = HashMap::new();
  map1.insert("key1".to_string(), "value1".to_string());
  map1.insert("key2".to_string(), "value2".to_string());

  let mut map2 = HashMap::new();
  map2.insert("key3".to_string(), "value3".to_string());
  map2.insert("key4".to_string(), "value4".to_string());

  let arg = [bjs::vec_hashmap_to_jsvalue(ctx, vec![map1, map2])];

  match bjs::default(ctx, format!("{root}/test.js"), &arg) {
    Ok(r) => {
      let r = bjs::li_str(ctx, r);
      dbg!(r);
    }
    Err(err) => {
      info!("{}", err);
    }
  }
  OK
}
```
