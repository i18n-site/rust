use aok::{Void, OK};
use upgrade_host::UPGRADE_HOST;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[tokio::test]
async fn test_async() -> Void {
  let uper = uper::conf::load(UPGRADE_HOST, true, "i18", [0, 1, 2]).await?;

  tokio::time::sleep(std::time::Duration::from_secs(3)).await;

  if let Some(uper) = uper {
    uper
      .join(
        std::fs::read("/Users/z/host/conf/env/upgrade/pk")?
          .try_into()
          .unwrap(),
      )
      .await?;
  }
  OK
}
