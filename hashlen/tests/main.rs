use aok::{OK, Void};
use hashlen::hashlen;
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  let hash = hashlen(b"134567890987654323456789876543222");
  info!("hash {:?} len {}", hash, hash.len());
  OK
}
