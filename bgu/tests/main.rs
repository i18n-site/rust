mod mirror;
use aok::{Result, OK};
use bgu::{boot, PUBLIC_KEY_LENGTH};
use static_init::constructor;

pub const PK: &[u8; PUBLIC_KEY_LENGTH] = include_bytes!("i18n.pk");

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

use bgu::ver;

async fn main() -> Result<()> {
  dbg!("test main");
  OK
}

#[tokio::test]
async fn test() -> Result<()> {
  boot(PK, "i18", ver!(), mirror::MIRROR, main).await?;
  OK
}
