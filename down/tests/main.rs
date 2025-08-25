use std::path::PathBuf;

use aok::{OK, Void};
use down::down;
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[tokio::test]
async fn test_async() -> Void {
  let file = "0.1.51/x86_64-unknown-linux-musl.tar";
  let tmp: PathBuf = format!("/tmp/{file}").into();
  std::fs::create_dir_all(tmp.parent().unwrap())?;

  let recv = down(
    [
      "github.com/up51/v/releases/download/i18-",
      "up0.u-01.eu.org/i18/",
      "up2.u-01.eu.org/i18/",
      "up3.u-01.eu.org/i18/",
      "yutk.eu.org/i18/",
    ]
    .map(|i| format!("https://{i}{file}")),
    &tmp,
  )
  .await?;
  if let Ok(size) = xerr::ok!(recv.recv().await) {
    while let Ok(info) = recv.recv().await {
      info!("{info}/{size}");
    }
  }

  info!("✅ {}", tmp.display().to_string());
  OK
}
