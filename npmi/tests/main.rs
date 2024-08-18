use aok::{Result, OK};
use npmi::Pkg;
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  for i in ["@3-/xx@1.0.0", "@3-/xx", "x@1.1.0", "x"] {
    let pkg = Pkg::new(i);
    dbg!((i, pkg));
  }

  let npm = npmi::Npm::new("/tmp/test");
  let pkg = "@typescript-eslint/typescript-estree";
  npm.i(pkg).await?;

  OK
}
