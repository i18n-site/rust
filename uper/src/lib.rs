#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

use std::{
  fs::File,
  path::{Path, PathBuf},
};

mod dir;
use pbar::pbar;
use sver::Ver;
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
  pub project: String,
  pub channel: String,
  pub ver: Ver,
}

fn tar_zstd_to_dir<P1: AsRef<Path>, P2: AsRef<Path>>(path: P1, to_dir: P2) -> std::io::Result<()> {
  let file = File::open(path)?;
  let decoder = zstd::stream::read::Decoder::new(file)?;
  let mut archive = tar::Archive::new(decoder);
  archive.unpack(to_dir)?;
  Ok(())
}

impl Uper {
  pub async fn load(
    project: impl Into<String>,
    channel: impl Into<String>,
    ver_url_li: VerUrlLi,
  ) -> Result<Self> {
    let project = project.into();
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
      ver: ver_url_li.ver,
      project,
    })
  }

  pub async fn join(self, pk: [u8; 32]) -> Void {
    let recv = self.recv;
    if let Ok(size) = xerr::ok!(recv.recv().await) {
      let mut pbar = pbar(size);
      pbar.set_message(format!("upgrade {} → {}", self.project, self.ver));
      let mut downed = 0;
      while let Ok(d) = recv.recv().await {
        downed = d;
        pbar.set_position(downed)
      }
      if downed == size {
        if let Some(verfiy_dir) = upgrade_verify::check(vb::e(self.ver.0), &self.file, pk)? {
          xerr::log!(std::fs::remove_file(&self.file));
          let tar_zst = verfiy_dir.join("tar.zst");

          let to_dir = dir::project(&self.project).join(format!("{}", self.ver));

          std::fs::remove_dir_all(&to_dir).ok();
          xerr::log!(tar_zstd_to_dir(&tar_zst, &to_dir));
          xerr::log!(std::fs::remove_dir_all(verfiy_dir));

          pbar.finish_with_message(format!(
            "✅ {} → {} ( {} channel)",
            self.project, self.ver, self.channel,
          ));
        }
      }
    }
    OK
  }
}
