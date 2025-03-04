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
  let root: PathBuf = "/Users/z/i18n/md".into();
  let yml_fp = root.join(".i18n").join("data").join("public").join("dev");

  let scan = change::Scan::new(root.join("public"))?;
  let change = scan.change(&yml_fp)?;
  change.save()?;
  OK
}
