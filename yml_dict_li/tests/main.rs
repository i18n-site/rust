use std::path::PathBuf;

use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let dir: PathBuf = env!("CARGO_MANIFEST_DIR").into();

  let li = yml_dict_li::set(dir.join("tests/test.yml"), "0.1.2", "1.2.4")?;
  dbg!(li);
  OK
}
