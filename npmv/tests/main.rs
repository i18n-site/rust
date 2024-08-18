use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  let pkg = "@3-/dbq";
  let v = npmv::latest(pkg).await?;
  info!("{}", v);
  npmv::tgz(pkg, v, "/tmp/test").await?;
  OK
}
