use aok::{OK, Result};
use tracing::info;

#[test]
fn test() -> Result<()> {
  loginit::init();
  info!("{}", s_::EMPTY);
  OK
}
