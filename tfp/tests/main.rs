use std::path::PathBuf;

use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

// #[tokio::test]
// async fn test() -> Result<()> {
//   info!("{}", 123456);
//   OK
// }

#[test]
fn test() -> Result<()> {
  let dir = env!("CARGO_MANIFEST_DIR");
  let dir: PathBuf = dir.into();
  let fp = dir.join("README.mdt");
  tfp::tfp(fp)?;
  OK
}
