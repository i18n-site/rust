use aok::{OK, Void};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

// #[tokio::test]
// async fn test_async() -> Void {
//   info!("async {}", 123456);
//   OK
// }

#[test]
fn test() -> Void {
  info!("> test {}", 123456);
  OK
}
