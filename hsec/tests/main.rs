use aok::{OK, Void};
use hsec::hsec;
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  info!("{}", hsec(99));
  info!("{}", hsec(999));
  info!("{}", hsec(9999));
  info!("{}", hsec(999999));
  OK
}
