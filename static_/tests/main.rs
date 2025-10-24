use aok::{OK, Result};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  log_init::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  static_::init().await?;
  OK
}
