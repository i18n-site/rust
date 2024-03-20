#![feature(trait_alias)]
#![feature(const_trait_impl)]
#![feature(effects)]

use std::{
  env::{consts::EXE_SUFFIX, temp_dir},
  io::Read,
  path::PathBuf,
};

use aok::Result;
pub use boot::boot;
pub use const_str;
use current_platform::CURRENT_PLATFORM;
pub use ed25519_dalek::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use ifs::{conf, dir::BIN_HOME, rsync};
use iget::Down;
use scopeguard::defer;
use tokio::{
  fs::{self, remove_dir_all},
  task::JoinHandle,
};

pub mod api {
  include!(concat!(env!("OUT_DIR"), "/api.rs"));
}

mod ver;
pub use ver::Ver;
mod boot;

pub const DO_UPDATE_DISABLE: i8 = -1;
pub const DO_UPDATE_FORCE: i8 = 1;

genv::def! {
  DO_UPDATE:i8 | 0;
  UPDATE_INTERVAL:u32 | 86400;
  UPDATE_OVERWRITE:bool | true;
}

pub fn exe_path(name: impl Into<String>, ver: &Ver) -> PathBuf {
  let mut path: std::path::PathBuf = (&*BIN_HOME).into();
  let name = name.into();
  path.push(&name);
  path.push(ver.to_string());
  path.push(name + EXE_SUFFIX);
  path
}

#[macro_export]
macro_rules! ver {
  () => {{
    use $crate::const_str::{parse, split};
    let r: [&str; 3] = split!(env!("CARGO_PKG_VERSION"), ".");
    $crate::Ver([parse!(r[0], u32), parse!(r[1], u32), parse!(r[2], u32)])
  }};
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

async fn bgu(
  pre_check: conf::Item<api::Ts>,
  name: impl Into<String>,
  now_ver: Ver,
  site: iget::Site,
) -> Result<Option<Downing>> {
  let name = name.into();
  let ver_txt = site.txt(format!("v/{name}")).await?;

  pre_check.set(api::Ts { v: sts::sec() });

  let mut ver_txt = ver_txt.as_str();
  if let Some(p) = ver_txt.find('\n') {
    ver_txt = &ver_txt[..p];
  }
  let ver: Ver = ver_txt.into();

  if now_ver >= ver {
    return Ok(None);
  }
  let tar = format!("{name}/{ver_txt}/{CURRENT_PLATFORM}.tar");
  let dir: String = temp_dir().as_os_str().to_string_lossy().into();

  let tar_fp = dir + &tar;

  Ok(Some(Downing {
    tar: tar_fp.clone(),
    ver,
    down: site.down(tar, tar_fp).await?,
  }))
}

pub struct Bgu<'a> {
  name: String,
  pk: &'a [u8; PUBLIC_KEY_LENGTH],
  ing: JoinHandle<Result<Option<Downing>>>,
}

// pub fn rand_new(prefix: impl Into<String>, li: &[(bool, impl AsRef<str>)]) -> Self {
//   let (h3, site) = li.choose(&mut rand::thread_rng()).unwrap();
//   let prefix = prefix.into();
//   (*h3, prefix + site.as_ref()).into()
// }

impl<'a> Bgu<'a> {
  pub fn new(
    pk: &'a [u8; PUBLIC_KEY_LENGTH],
    name: impl Into<String>,
    ver: Ver,
    li: &[(bool, impl AsRef<str>)],
  ) -> Option<Self> {
    let do_update = DO_UPDATE();
    if do_update == DO_UPDATE_DISABLE {
      return None;
    }

    let name = name.into();
    let conf = Into::<Conf>::into(BIN_HOME.join(&name));
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

    let site = {
      use rand::seq::SliceRandom;
      let (h3, url) = li.choose(&mut rand::thread_rng()).unwrap();
      let url = "https://".to_owned() + url.as_ref();
      iget::Site::new(*h3, url)
    };

    Some(Self {
      name: name.clone(),
      ing: tokio::spawn(bgu(pre_check, name, ver, site)),
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
              let mut bin_dir: PathBuf = BIN_HOME.clone();
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
            let conf_ver = Into::<Conf>::into(BIN_HOME.join(&name)).ver();
            if UPDATE_OVERWRITE() {
              #[allow(clippy::never_loop)]
              loop {
                if let Ok(current_exe) = xerr::ok!(std::env::current_exe()) {
                  let current_dir = current_exe
                    .parent()
                    .map(|i| i.as_os_str().to_string_lossy().to_string())
                    .unwrap_or_else(|| "/".to_owned());
                  let bin_home = BIN_HOME.as_os_str().to_string_lossy();
                  if current_dir.contains(bin_home.as_ref()) && current_dir != bin_home {
                    break;
                  }
                  if xerr::is_ok!(rsync(&bin_dir, current_dir)) {
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
