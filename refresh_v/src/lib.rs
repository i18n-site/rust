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

          let mut retry = 0;
          loop {
            if retry > 30 {
              eprintln!("❌ {pkg} version refresh failed");
              break;
            }
            retry += 1;
            let r_ver = req.get(pkg).await?;
            let r_ver = String::from_utf8_lossy(&r_ver);
            if new_ver != r_ver {
              eprintln!(
                "{retry} : {} version refresh: remote {} != {} ; wait for 3s",
                pkg, r_ver, new_ver
              );
              sleep(Duration::from_secs(3)).await;
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
// }
