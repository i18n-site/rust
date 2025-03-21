use aok::{OK, Void};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  for i in [0, 1, 300, u64::MAX] {
    let r36 = r36::e(i);
    let d = u64::from_str_radix(&r36, 36)?;
    info!("> {} {} {}", i, r36, d);
    assert_eq!(i, d);
  }
  OK
}
