use aok::{OK, Result};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  // let token = npm::token();
  // if token.is_empty() {
  //   info!("npm token is empty");
  // } else {
  //   let dir: PathBuf = env!("CARGO_MANIFEST_DIR").into();
  //   let src = dir.join("tests").join("pkg");
  //   let package_json = src.join("package.json");
  //   info!("begin publish");
  //   // xerr::log!(npm::publish(&token, src, &package_json).await);
  // }
  // info!("done");
  OK
}
