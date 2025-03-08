use std::path::PathBuf;

use tracing::info;
use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let root: PathBuf = env!("CARGO_MANIFEST_DIR").into();
  info!("root {}", root.display());
  let yml_fp = root.join("tests").join("state.yml");
  let scan = change::Scan::new(root)?;
  let diff = scan.diff(&yml_fp)?;
  info!("has_change {}", diff.has_change);
  for (fp, _meta) in &diff.changed {
    info!("{}", fp);
  }
  diff.save()?;
  OK
}
