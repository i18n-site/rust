use aok::{OK, Result};
use static_init::constructor;
use ver_count::VerCount;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

// #[tokio::test]
// async fn test() -> Result<()> {
//   info!("{}", 123456);
//   OK
// }

#[test]
fn test() -> Result<()> {
  let ver_li = ["0.1.0", "0.1.3", "0.1.2", "0.1.2"];
  let mut vc = VerCount::default();
  for i in ver_li {
    vc.push(i);
  }
  println!("{:?}", vc.map());
  OK
}
