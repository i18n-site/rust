use std::sync::atomic::{AtomicU8, Ordering};

use aok::{OK, Result};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  let n = AtomicU8::new(0);
  tryn::retry(|| async {
    let t = n.fetch_add(1, Ordering::SeqCst);
    if t == 0 {
      aok::throw!("test error");
    }
    Ok(())
  })
  .await?;

  OK
}
