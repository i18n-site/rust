use aok::{OK, Void};
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
  for i in [
    "https://github.com/up51/v/releases/download/i18-0.1.36/aarch64-apple-darwin.tar",
    "https://d.u-01.eu.org/i18/0.1.36/aarch64-apple-darwin.tar",
    "https://f.u-01.eu.org/i18/0.1.36/aarch64-apple-darwin.tar",
  ] {
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
