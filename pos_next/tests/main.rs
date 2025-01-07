use aok::{Result, OK};
use pos_next::PosNext;
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let p = PosNext::new();
  info!("{}", p.next());
  info!("{}", p.next());
  info!("{}", p.next());
  info!("{}", p.next());
  OK
}
