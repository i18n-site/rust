use aok::{OK, Void};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  let n: u64 = 123456;
  let bin = intbin::to_bin(n);
  info!("{n} -> {:?}", &bin);
  assert_eq!(n, intbin::bin_u64(bin));

  OK
}
