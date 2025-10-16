use aok::{OK, Void};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  for i in riter::iter(&[1, 2, 3, 4, 5, 6, 7, 8, 9]) {
    info!(i)
  }
  OK
}
