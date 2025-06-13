use aok::{Result, OK};
use tracing::info;

#[test]
fn test() -> Result<()> {
  loginit::init();
  info!("{}", s_::EMPTY);
  OK
}
