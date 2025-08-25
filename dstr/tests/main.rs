use aok::{OK, Result};
use dstr::{dstr, dvec};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[derive(Debug)]
pub struct T {
  pub i: i32,
}

#[test]
fn test() -> Result<()> {
  let a = T { i: 123 };
  info!("{}", dstr(&a));
  let b = T { i: 456 };
  info!("{:?}", dvec![a, b]);
  OK
}
