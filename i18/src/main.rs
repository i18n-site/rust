use aok::{Result, OK};
use bgu::{ver, PUBLIC_KEY_LENGTH};
use i18::{mirror, run};

pub const PK: &[u8; PUBLIC_KEY_LENGTH] = include_bytes!("i18n.pk");

#[tokio::main]
async fn main() -> Result<()> {
  bgu::boot(PK, "i18", ver!(), mirror::MIRROR, run).await?;
  OK
}
