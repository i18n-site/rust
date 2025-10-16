use std::{
  fs,
  fs::File,
  path::{Path, PathBuf},
};

use aok::{OK, Result, Void};
use current_platform::CURRENT_PLATFORM as TARGET;
use defer_lite::defer;
use down::down;
use kanal::AsyncReceiver;
use pbar::pbar;
use self_replace::self_replace;
use sver::Ver;
use ver_from_txt::VerUrlLi;

#[derive(Debug)]
pub struct Uper {
  pub recv: AsyncReceiver<u64>,
  pub path: PathBuf,
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

    let path = tmpdir.join(TARGET);

    if path.exists() {
      if path.is_dir() {
        fs::remove_dir_all(&path)?;
      } else {
        fs::remove_file(&path)?;
      }
    }

    let recv = down(
      ver_url_li
        .url_li
        .into_iter()
        .map(|i| format!("{i}/{TARGET}.tar")),
      &path,
    )
    .await?;
    Ok(Self {
      recv,
      path,
      channel,
      ver: ver_url_li.ver,
      project,
    })
  }

  pub async fn join(self, pk: [u8; 32]) -> Void {
    let recv = self.recv;
    let path = &self.path;

    if let Ok(size) = xerr::ok!(recv.recv().await) {
      let mut pbar = pbar(size);
      pbar.set_message(format!("upgrade {} → {}", self.project, self.ver));
      let mut downed = 0;
      while let Ok(d) = recv.recv().await {
        downed = d;
        pbar.set_position(downed)
      }
      let ver = self.ver;
      if downed == size
        && let Some(verfiy_dir) = upgrade_verify::check(vb::e(ver.0), path, pk)?
      {
        xerr::log!(std::fs::remove_file(path));
        let tar_zst = verfiy_dir.join("tar.zst");
        xerr::log!(tar_zstd_to_dir(&tar_zst, path));
        xerr::log!(std::fs::remove_dir_all(verfiy_dir));

        pbar.finish_and_clear();

        let exe = path.join(format!("{}{}", &self.project, std::env::consts::EXE_SUFFIX));

        match std::process::Command::new(&exe).args(["-v"]).output() {
          Ok(r) => {
            let stdout = String::from_utf8_lossy(&r.stdout);
            let stdout = stdout.trim();
            if r.status.success() {
              defer! {
                xerr::log!(std::fs::remove_dir_all(path));
              }

              self_replace(&exe)?;
              let down_ver: Ver = stdout.into();
              if down_ver == ver {
                println!("✅ {} → {} ( {} channel)", self.project, ver, self.channel,);
              } else {
                eprintln!("❌ down version {down_ver} != {ver}",);
              }
            } else {
              println!("{stdout}");
              eprintln!("{}", String::from_utf8_lossy(&r.stderr));
            }
          }
          Err(err) => {
            eprintln!("❌ {err}");
          }
        };
      }
    }
    OK
  }
}
