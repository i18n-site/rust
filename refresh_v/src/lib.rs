use aok::{Result, OK};
use mreq::Mreq;
use tokio::time::{sleep, Duration};

pub async fn url(
  token: impl AsRef<str>,
  url: impl AsRef<str>,
  new_ver: impl AsRef<str>,
) -> Result<()> {
  let token = token.as_ref();
  let url = url.as_ref();
  let new_ver = new_ver.as_ref();
  if let Some(p) = url.find("//") {
    let url = &url[p + 2..];
    if let Some(p) = url.find("/") {
      if url.len() > (1 + p) {
        let host = &url[..p];
        let pkg = &url[p + 1..];
        if !&pkg[1..].contains("@") && ["unpkg.com"].contains(&host) {
          let mut req = Mreq::new(["upv.i18n.site", "upv.x01.site"], [("t", token)]);

          let mut max_retry = 9;
          while max_retry > 0 {
            max_retry -= 1;
            if new_ver != req.get(pkg).await? {
              eprintln!("❌ .v refresh {} != {}", pkg, new_ver);
              sleep(Duration::from_secs(1)).await;
            } else {
              break;
            }
          }
        }
      }
    }
  }
  OK
}

// pub async fn refresh_v(token: impl AsRef<str>, new_ver: impl AsRef<str>) {
//   let token = token.as_ref();
//   let new_ver = new_ver.as_ref();
//   dbg!(token, new_ver);
// }
