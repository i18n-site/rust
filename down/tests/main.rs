use aok::{Void, OK};
use down::{down, meta};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[tokio::test]
async fn test_async() -> Void {
  let mut url_li = vec![];
  let mut filesize = 0;

  let file = "0.1.41/x86_64-unknown-linux-musl.tar";
  for i in [
    "https://github.com/up51/v/releases/download/i18-",
    "https://up0.u-01.eu.org/i18/",
    "https://up2.u-01.eu.org/i18/",
    "https://up3.u-01.eu.org/i18/",
    "https://yutk.eu.org/i18/",
  ]
  .map(|i| format!("{i}{file}"))
  {
    let (size, url) = meta(i).await?;
    info!("{} {}", size, url.to_string());
    if size > 0 {
      filesize = size;
      url_li.push(url);
    }
  }

  down(filesize, url_li, "/tmp/i18.tar").await?;

  OK
}
