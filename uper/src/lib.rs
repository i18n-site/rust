#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

use std::path::PathBuf;

use kanal::AsyncReceiver;
use aok::{OK, Result, Void};
pub use const_str;
use ver_from_txt::VerUrlLi;
use current_platform::CURRENT_PLATFORM as TARGET;
use down::down;

pub mod conf;
pub mod dns_check;
use dns_check::dns_check;

#[derive(Debug)]
pub struct Uper {
  pub recv: AsyncReceiver<u64>,
  pub file: PathBuf,
  pub channel: String,
  pub ver: [u64; 3],
}

impl Uper {
  pub async fn load(
    project: impl AsRef<str>,
    channel: impl Into<String>,
    ver_url_li: VerUrlLi,
  ) -> Result<Self> {
    let project = project.as_ref();
    let channel = channel.into();
    let tmpdir = std::env::temp_dir().join(format!("uper/{project}/{}", ver_url_li.ver));
    std::fs::create_dir_all(&tmpdir)?;
    let filename = format!("{TARGET}.tar");
    let file = tmpdir.join(&filename);

    let recv = down(
      ver_url_li
        .url_li
        .into_iter()
        .map(|i| format!("{i}/{filename}")),
      &file,
    )
    .await?;
    Ok(Self {
      recv,
      file,
      channel,
      ver: ver_url_li.ver.0,
    })
  }

  pub async fn join(self, pk: [u8; 32]) -> Void {
    let recv = self.recv;
    if let Ok(size) = xerr::ok!(recv.recv().await) {
      while let Ok(info) = recv.recv().await {
        tracing::info!("{info}/{size}");
      }
    }
    if let Some(fp) = upgrade_verify::check(vb::e(self.ver), &self.file, pk)? {
      std::fs::remove_file(&self.file)?;
      let tar_zst = fp.join("tar.zst");
      tracing::info!("下载完成 {}", tar_zst.display());
    }
    OK
  }
}
