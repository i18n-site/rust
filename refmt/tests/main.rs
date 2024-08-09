use aok::{Result, OK};
use static_init::constructor;

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
  let txt = "123 \r\n45\r 6  \n\n";

  let r = refmt::str(txt);
  assert_eq!(r, ("123\n45\n 6".to_owned(), true));
  dbg!(r);

  // let dir = env!("CARGO_MANIFEST_DIR");
  // let dir: PathBuf = dir.into();
  // let fp = dir.join("README.mdt");
  // refmt::refmt(fp)?;
  OK
}
