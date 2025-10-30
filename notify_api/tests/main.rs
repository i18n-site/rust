use aok::{OK, Void};

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[tokio::test]
async fn test_async() -> Void {
  notify_api::send(
    "rust/notify_api 标题",
    "正文",
    "https://crates.io/crates/notify_api",
  )
  .await?;
  OK
}
