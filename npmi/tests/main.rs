use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  let pkg = npmi::PkgLi::new("/tmp/test", &["@typescript-eslint/typescript-estree"]);
  pkg.auto().await?;
  // npm.i(pkg).await?;

  OK
}
