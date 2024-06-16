use std::path::PathBuf;

use aok::{Result, OK};
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::main]
async fn main() -> Result<()> {
  let token = npm::token();
  if token.is_empty() {
    info!("npm token is empty");
  } else {
    let dir: PathBuf = env!("CARGO_MANIFEST_DIR").into();
    let src = dir.join("pkg");
    let package_json = src.join("package.json");
    npm::publish(&token, src, &package_json).await?;
  }
  info!("done");
  OK
}
