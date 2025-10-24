use aok::{OK, Void};
use log::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  log_init::init();
}

#[test]
fn test() -> Void {
  info!(">>>>> {}", 123456);
  OK
}
