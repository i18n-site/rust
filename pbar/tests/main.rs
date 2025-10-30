use aok::{OK, Result};
use pbar::pbar;
use static_init::constructor;
use tokio::time::Duration;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  let mut pbar = pbar(100);
  for i in 0..100 {
    if i % 5 == 0 {
      pbar.set_message(format!("set message {}", i));
    }
    pbar.inc(1);
    tokio::time::sleep(Duration::from_millis(50)).await;
  }
  OK
}
