use anyhow::Result;
use log::info;

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
}

// #[tokio::test]
// async fn test_async() -> Result<()>{ {
//   info!("async {}", 123456);
//   Ok(())
// }

#[test]
fn test() -> Result<()> {
  info!("> test {}", 123456);
  Ok(())
}
