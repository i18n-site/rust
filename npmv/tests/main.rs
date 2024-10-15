use aok::{Result, OK};
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  let pkg = "@3-/dbq";
  let dir = "/tmp";
  let result = npmv::cache::latest(pkg, &dir).await?;
  info!("{} {}", result, dir);
  OK
}
