use aok::OK;
use mreq::Mreq;
use tokio::time::{sleep, Duration};
use waiting::Waiting;

pub struct RefreshV {
  pkg: String,
  recv: kanal::Receiver<String>,
}

impl RefreshV {
  pub fn wait(&self) {
    let pkg = &self.pkg;
    let ing = Waiting::new(format!("refresh {} version", pkg));
    if let Ok(ver) = self.recv.recv() {
      ing.end(format!("✅ {pkg} v{ver}"));
    }
  }

  pub fn run(
    token: impl Into<String>,
    url: impl AsRef<str>,
    new_ver: impl Into<String>,
  ) -> Option<Self> {
    let token = token.into();
    let url = url.as_ref();
    let new_ver = new_ver.into();
    if let Some(p) = url.find("//") {
      let url: String = url[p + 2..].into();
      if let Some(p) = url.find("/") {
        if url.len() > (1 + p) {
          let host = &url[..p];
          let pkg_str = &url[p + 1..];
          let pkg: String = pkg_str.into();
          if !&pkg[1..].contains("@") && ["unpkg.com"].contains(&host) {
            let (send, recv) = kanal::bounded(1);

            tokio::spawn(async move {
              let mut req = Mreq::new(["upv.i18n.site", "upv.x01.site"], [("t", token.as_str())]);
              let mut retry = 0;
              loop {
                if retry > 30 {
                  eprintln!("❌ {pkg} version refresh failed");
                  break;
                }
                retry += 1;
                let r_ver = req.get(&pkg).await?;
                let r_ver = String::from_utf8_lossy(&r_ver);
                if new_ver != r_ver {
                  eprintln!(
                    "{retry} : {} version refresh: remote {} != {} ; wait for 3s",
                    pkg, r_ver, new_ver
                  );
                  sleep(Duration::from_secs(3)).await;
                } else {
                  send.send(r_ver.into())?;
                  break;
                }
              }
              OK
            });
            return Some(RefreshV {
              recv,
              pkg: pkg_str.into(),
            });
          }
        }
      }
    }
    None
  }
}

// pub async fn refresh_v(token: impl AsRef<str>, new_ver: impl AsRef<str>) {
//   let token = token.as_ref();
//   let new_ver = new_ver.as_ref();
// }
