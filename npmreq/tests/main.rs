use aok::{Result, OK};
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  let pkg = npmreq::Pkg::new("@3-/dbq");
  let v = pkg.latest().await?;
  info!("{}", v);
  pkg.tgz(v, "/tmp/test").await?;
  OK
}
