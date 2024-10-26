#![feature(trait_alias)]
#![feature(const_trait_impl)]
use std::{
  env::{consts::EXE_SUFFIX, temp_dir},
  io::Read,
  path::PathBuf,
};

use aok::Result;
pub use boot::boot;
use current_platform::CURRENT_PLATFORM;
use defer_lite::defer;
pub use ed25519_dalek::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use ifs::{conf, dir::CACHE, rsync, rsync::WalkDir};
use iget::Down;
use mreq::Mreq;
use tokio::{
  fs::{self, remove_dir_all},
  task::JoinHandle,
};

pub mod api {
  include!(concat!(env!("OUT_DIR"), "/api.rs"));
  impl From<Ver> for sver::Ver {
    fn from(ver: Ver) -> Self {
      Self([ver.major, ver.minor, ver.patch])
    }
  }
}

pub use sver::Ver;
mod boot;
mod randiter;

pub const DO_UPDATE_DISABLE: i8 = -1;
pub const DO_UPDATE_FORCE: i8 = 1;

genv::def! {
  DO_UPDATE:i8 | 0;
  UPDATE_INTERVAL:u32 | 86400;
  UPDATE_OVERWRITE:bool | true;
}

pub fn exe_path(name: impl Into<String>, ver: &Ver) -> PathBuf {
  let mut path: std::path::PathBuf = (&*CACHE).into();
  let name = name.into();
  path.push(&name);
  path.push(ver.to_string());
  path.push(name + EXE_SUFFIX);
  path
}

// impl From<St> for Ver {
//   type Error = vb::Error;
//
//   fn try_from(bin: &[u8]) -> Result<Self, Self::Error> {
//     let v = vb::d(bin)?;
//     let len = v.len();
//     if v.len() != 3 {
//       tracing::warn!("ver.len != 3 > {:?}", bin);
//     }
//     // major.minor.patch
//     let major = if len > 0 { v[0] as _ } else { 0 };
//     let minor = if len > 1 { v[1] as _ } else { 0 };
//     let patch = if len > 2 { v[2] as _ } else { 0 };
//     Ok(Ver(major, minor, patch))
//   }
// }

conf!(Conf {
  preCheck: Ts,
  ver: Ver
});

pub struct Downing {
  pub ver: Ver,
  pub down: Down,
  pub tar: String,
}

async fn _bgu(url: String, name: &str, now_ver: &Ver, new_ver: &Ver) -> Result<Option<Downing>> {
  let site = {
    let url = "https://".to_owned() + url.as_ref();
    iget::Site::new(url)
  };

  println!("UPGRADE {now_ver} → {new_ver}");
  let tar = format!("{name}/{new_ver}/{CURRENT_PLATFORM}.tar");
  let dir: String = temp_dir().as_os_str().to_string_lossy().into();

  let tar_fp = dir + &tar;
  let down = site.down(tar, tar_fp.clone()).await?;
  Ok(Some(Downing {
    tar: tar_fp,
    ver: new_ver.clone(),
    down,
  }))
}

async fn bgu(
  pre_check: conf::Item<api::Ts>,
  name: impl Into<String>,
  now_ver: Ver,
  v_li: Vec<String>,
  mut li: Vec<String>,
) -> Result<Option<Downing>> {
  let mut req = Mreq::new(v_li, []);
  let name = name.into();
  let new_ver = req.get(format!("v/{name}")).await?;

  defer! {
    pre_check.set(api::Ts { v: sts::sec() });
  }

  let mut new_ver = new_ver.as_ref();
  if let Some(p) = new_ver.iter().position(|&b| b == b'\n') {
    new_ver = &new_ver[..p];
  }
  let new_ver: Ver = String::from_utf8_lossy(new_ver).into();

  if now_ver >= new_ver {
    return Ok(None);
  }

  while let Some(url) = li.pop() {
    if let Ok(r) = xerr::ok!(_bgu(url, &name, &now_ver, &new_ver).await) {
      return Ok(r);
    }
  }
  Ok(None)
}

pub struct Bgu<'a> {
  name: String,
  pk: &'a [u8; PUBLIC_KEY_LENGTH],
  ing: JoinHandle<Result<Option<Downing>>>,
}

impl<'a> Bgu<'a> {
  pub fn new(
    pk: &'a [u8; PUBLIC_KEY_LENGTH],
    name: impl Into<String>,
    ver: Ver,
    v_li: &[&str],
    down_li: &[&str],
  ) -> Option<Self> {
    let do_update = DO_UPDATE();
    if do_update == DO_UPDATE_DISABLE {
      return None;
    }

    let name = name.into();
    let conf = Into::<Conf>::into(CACHE.join(&name));
    let pre_check = conf.preCheck();
    if do_update != DO_UPDATE_FORCE {
      let now = sts::sec();
      if let Some(pre_check) = pre_check.get() {
        if now > pre_check.v {
          let diff = now - pre_check.v;
          if diff < UPDATE_INTERVAL() as u64 {
            return None;
          }
        }
      }
    }

    Some(Self {
      name: name.clone(),
      ing: tokio::spawn(bgu(
        pre_check,
        name,
        ver,
        v_li.iter().map(|i| i.to_string()).collect(),
        randiter::RandIter::new(down_li)
          .map(|url| url.to_string())
          .collect(),
      )),
      pk,
    })
  }

  pub async fn join(self) -> Result<Option<Ver>> {
    if let Some(ing) = self.ing.await?? {
      return down(self.name, self.pk, ing).await;
    }
    Ok(None)
  }
}

pub async fn down(name: String, pk: &[u8; PUBLIC_KEY_LENGTH], ing: Downing) -> Result<Option<Ver>> {
  use std::{fs::File, io::BufReader};

  use ed25519_dalek::{Signature, Verifier, VerifyingKey};
  use ifs::txz_hash_d as txz;

  let tar = ing.tar;
  ing.down.show().await?;

  defer! {
    let tar = PathBuf::from(&tar);
    xerr::log!(std::fs::remove_dir_all(tar.parent().unwrap()));
  }

  let mut hsc = [0u8; SIGNATURE_LENGTH];
  let mut hash = None;

  for entry in txz::Tar::new(BufReader::new(File::open(&tar)?)).entries()? {
    let mut entry = entry?;
    if let Ok(path) = entry.path() {
      if let Some(ext) = path.extension() {
        if let Some(ext) = ext.to_str() {
          match ext {
            "hsc" => {
              entry.read_exact(&mut hsc[..])?;
            }
            "txz" => {
              let mut bin_dir: PathBuf = CACHE.clone();
              bin_dir.push(&name);
              let path: PathBuf = path.into();
              if let Some(path) = path.iter().next_back() {
                let path = path.to_string_lossy();
                if let Some(p) = path.rfind('.') {
                  bin_dir.push(&path[..p])
                }
              };
              let hasher: blake3::Hasher = txz::d(&mut entry, bin_dir.clone())?;
              hash = Some((hasher.finalize(), bin_dir));
            }
            _ => {}
          }
        }
      }
    }
  }

  if let Some((hash, bin_dir)) = hash {
    let verify = VerifyingKey::from_bytes(pk)?;
    let sign = Signature::from_bytes(&hsc);
    if xerr::is_ok!(verify.verify(hash.as_bytes(), &sign)) {
      let exe_name = name.to_owned() + std::env::consts::EXE_SUFFIX;
      let exe = bin_dir.join(&exe_name);
      use std::process::Command;
      let out = Command::new(&exe).args(["-v"]).output()?;
      if out.status.success() {
        let out = String::from_utf8(out.stdout)?;
        if let Some(ver_line) = out.lines().next() {
          let mut t = Vec::new();

          for i in ver_line.chars().rev() {
            if i == '.' || i.is_ascii_digit() {
              t.push(i);
            } else {
              break;
            }
          }

          let ver: Ver = t.into_iter().rev().collect::<String>().into();
          let down_ver = ing.ver;
          if ver == down_ver {
            let conf_ver = Into::<Conf>::into(CACHE.join(&name)).ver();
            if UPDATE_OVERWRITE() {
              #[allow(clippy::never_loop)]
              loop {
                if let Ok(current_exe) = xerr::ok!(std::env::current_exe()) {
                  let current_dir = current_exe
                    .parent()
                    .map(|i| i.as_os_str().to_string_lossy().to_string())
                    .unwrap_or_else(|| "/".to_owned());
                  let bin_home = CACHE.as_os_str().to_string_lossy();
                  if current_dir.contains(bin_home.as_ref()) && current_dir != bin_home {
                    break;
                  }
                  if xerr::is_ok!(rsync(&bin_dir, WalkDir::new(&bin_dir), current_dir)) {
                    if let Some(current_exe_name) = current_exe.file_name() {
                      let current_exe_name = current_exe_name.to_string_lossy();
                      if current_exe_name != exe_name {
                        let mut rename_from = current_exe.clone();
                        rename_from.set_file_name(exe_name);
                        if let Err(err) = fs::rename(&rename_from, &current_exe).await {
                          tracing::error!(
                            "{} {} → {}",
                            err,
                            rename_from.display(),
                            current_exe.display(),
                          );
                          break;
                        }
                      }
                    }
                    conf_ver.rm();
                    xerr::log!(remove_dir_all(bin_dir).await);
                    return Ok(Some(ver));
                  }
                }
              }
            }
            let ver0 = ver.0;
            conf_ver.set(api::Ver {
              major: ver0[0],
              minor: ver0[1],
              patch: ver0[2],
            });
            return Ok(Some(ver));
          } else {
            tracing::error!("{} bin ver {ver} != down ver {down_ver}", name);
          }
        }
      }
    } else {
      xerr::log!(remove_dir_all(bin_dir).await);
    }
  }
  Ok(None)
}
