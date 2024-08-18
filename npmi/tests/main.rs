use aok::{Result, OK};
use npmi::Pkg;
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  for i in ["@3-/xx@1.0.0", "@3-/xx", "x@1.1.0", "x"] {
    let pkg = Pkg::new(i);
    dbg!((i, pkg));
  }
  info!("{}", 123456);
  OK
}
