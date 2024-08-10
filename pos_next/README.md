[‼️]: ✏️README.mdt

# pos_next

一个不同线程之间可以共享的 pos , 用了不安全代码, 不保证数字的连续性

```rust
use aok::{Result, OK};
use pos_next::PosNext;
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let p = PosNext::new();
  info!("{}", p.next());
  info!("{}", p.next());
  info!("{}", p.next());
  info!("{}", p.next());
  OK
}
```
