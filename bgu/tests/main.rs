mod host;
use aok::{Result, OK};
use bgu::{boot, PUBLIC_KEY_LENGTH};
use static_init::constructor;

pub const PK: &[u8; PUBLIC_KEY_LENGTH] = include_bytes!("i18n.pk");

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

async fn main() -> Result<()> {
  OK
}

#[tokio::test]
async fn test() -> Result<()> {
  boot(PK, host::V_HOST, host::MIRROR, "i18", [0, 0, 0], main).await?;

  OK
}
