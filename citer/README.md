[‼️]: ✏️README.mdt

# citer

```rust
use aok::{Result, OK};
use citer::CIter;
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
  let vec = &[1, 2, 3, 4][..];
  let iter = CIter::new(vec, 1);

  for value in iter {
    println!("{}", value);
  }
  OK
}
```
