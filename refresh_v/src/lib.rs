use aok::OK;
use mreq::Mreq;
use tokio::time::{Duration, sleep};
use waiting::Waiting;

pub struct RefreshV {
  pkg: String,
  recv: kanal::Receiver<String>,
}

impl RefreshV {
  pub fn wait(&self) {
    let pkg = &self.pkg;
    let ing = Waiting::new(format!("refresh {pkg} version"));
    if let Ok(ver) = self.recv.recv() {
      ing.end(format!("✅ {pkg} v{ver}"));
    }
  }

  pub fn run(token: impl Into<String>, pkg: impl Into<String>, new_ver: impl Into<String>) -> Self {
    let token = token.into();
    let pkg = pkg.into();
    let new_ver = new_ver.into();
    let (send, recv) = kanal::bounded(1);

    let _pkg = pkg.clone();
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
          eprintln!("{retry} : {pkg} version refresh: remote {r_ver} != {new_ver} ; wait for 3s");
          sleep(Duration::from_secs(3)).await;
        } else {
          send.send(r_ver.into())?;
          break;
        }
      }
      OK
    });

    RefreshV { recv, pkg: _pkg }
  }
}

// pub async fn refresh_v(token: impl AsRef<str>, new_ver: impl AsRef<str>) {
//   let token = token.as_ref();
//   let new_ver = new_ver.as_ref();
// }
