use aok::{OK, Result};
use log::info;
use static_init::constructor;
use xbin::concat;

#[constructor(0)]
extern "C" fn init() {
  log_init::init()
}

#[test]
fn test() -> Result<()> {
  let s1 = "123";
  let s2 = [4u8, 5, 6];
  let s3 = vec![7u8, 8, 9];
  let result = concat!(s1, s2, s3);
  assert_eq!(result, b"123	");

  info!("test ok");

  OK
}
