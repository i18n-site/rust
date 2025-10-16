use aok::{OK, Result};
use pos_next::PosNext;
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let p = PosNext::new(1000);
  info!("{}", p.next());
  info!("{}", p.next());
  info!("{}", p.next());
  info!("{}", p.next());
  OK
}
