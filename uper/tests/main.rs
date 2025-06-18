use aok::{OK, Void};
use upgrade_host::UPGRADE_HOST;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

pub async fn run() -> Void {
  dbg!("!!run");
  OK
}

#[tokio::test]
async fn test_async() -> Void {
  uper::conf::load(
    UPGRADE_HOST,
    std::fs::read("/Users/z/host/conf/env/upgrade/pk")?
      .try_into()
      .unwrap(),
    true,
    run,
    "i18",
    [0, 1, 2],
  )
  .await
}
