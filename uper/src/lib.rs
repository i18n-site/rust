#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

use std::sync::Arc;

use tokio::sync::RwLock;
pub use const_str;
use tracing::warn;
use ver_from_txt::VerUrlLi;
use aok::{OK, Void};
use current_platform::CURRENT_PLATFORM as TARGET;

pub mod conf;
pub mod dns_check;
use dns_check::dns_check;

#[derive(Debug)]
pub struct Uper {
  recv: kanal::AsyncReceiver<String>,
}

pub fn run(
  project: impl AsRef<str>,
  channel: impl AsRef<str>,
  send: kanal::AsyncSender<String>,
  ver_url_li: VerUrlLi,
) {
  let msg = format!(
    "\n{} upgrading → {} ( channel {} )",
    project.as_ref(),
    ver_url_li.ver,
    channel.as_ref()
  );
  tokio::spawn(async move {
    let size = Arc::new(RwLock::new(0u64));

    send.send(msg).await?;

    let (send_url, recv_url) = kanal::bounded_async(1);

    for url in ver_url_li
      .url_li
      .iter()
      .map(|i| format!("{i}/{TARGET}.tar"))
    {
      let send_url = send_url.clone();
      let size = size.clone();
      tokio::spawn(async move {
        match down::meta(&url).await {
          Ok((filesize, url)) => {
            if filesize > 0 {
              loop {
                let ptr = size.read().await;
                let size_u64 = *ptr;
                if size_u64 == 0 {
                  drop(ptr);
                  let mut ptr = size.write().await;
                  // 获取写锁后，必须再次检查！因为在释放读锁和获取写锁的间隙，其他任务可能已经写入了值。
                  if *ptr == 0 {
                    *ptr = filesize;
                  } else {
                    continue;
                  }
                } else if size_u64 != filesize {
                  warn!("{url} filesize {filesize} != other {size_u64}");
                }
                break;
              }
              send_url.send(url).await?;
            } else {
              warn!("upgrade {url} filesize == 0")
            }
          }
          Err(e) => {
            warn!("upgrade {url} {e}");
          }
        }
        OK
      });
    }

    // 确保recv会释放
    drop(send_url);

    let mut filesize = 0;
    while let Ok(url) = recv_url.recv().await {
      if filesize == 0 {
        filesize = *size.read().await;
        dbg!(filesize);
      }
      dbg!(url.to_string());
      // send.send(format!("{filesize} → {url}")).await?;
    }
    OK
  });
}

impl Uper {
  pub fn new(project: impl AsRef<str>, channel: impl AsRef<str>, ver_url_li: VerUrlLi) -> Self {
    let (send, recv) = kanal::unbounded_async();
    run(project, channel, send, ver_url_li);
    Self { recv }
  }

  pub async fn join(&self) -> Void {
    while let Ok(msg) = self.recv.recv().await {
      println!("{msg}");
    }
    OK
  }
}
