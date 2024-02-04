mod mirror;
use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

use bgu::{bgu, ver};

#[tokio::test]
async fn test() -> Result<()> {
  bgu(ver!(), &mirror::MIRROR).await?;
  OK
}
