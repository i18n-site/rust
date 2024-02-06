use std::{
  env::temp_dir,
  fmt::{Debug, Display},
  io::Read,
  path::PathBuf,
};

use aok::Result;
pub use const_str;
use current_platform::CURRENT_PLATFORM;
pub use ed25519_dalek::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use iget::Down;
use scopeguard::defer;
use tokio::{fs::remove_dir_all, task::JoinHandle};

#[derive(Default, PartialOrd, Ord, PartialEq, Eq)]
pub struct Ver(pub [u32; 3]);

genv::def!(
  XDG_BIN_HOME:PathBuf |
  dirs::home_dir().map(
    |i|i.join(".local/bin")
  ).unwrap_or_else(||{
    std::env::current_exe().unwrap().parent().unwrap().into()
  })
);

impl Display for Ver {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let li = self.0;
    write!(f, "{}.{}.{}", li[0], li[1], li[2])
  }
}

impl Debug for Ver {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let li = self.0;
    write!(f, "Ver({}.{}.{})", li[0], li[1], li[2])
  }
}

#[macro_export]
macro_rules! ver {
  () => {{
    use $crate::const_str::{parse, split};
    let r: [&str; 3] = split!(env!("CARGO_PKG_VERSION"), ".");
    $crate::Ver([parse!(r[0], u32), parse!(r[1], u32), parse!(r[2], u32)])
  }};
}

impl<S: AsRef<str>> From<S> for Ver {
  fn from(s: S) -> Self {
    let mut s = s.as_ref();
    macro_rules! parse {
      () => {{
        if let Some(p) = s.find('.') {
          let r = s[..p].parse().unwrap_or(0);
          s = &s[p + 1..];
          r
        } else {
          0
        }
      }};
    }
    let major = parse!();
    let minor = parse!();
    let patch = s.parse().unwrap_or(0);
    Ver([major, minor, patch])
  }
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

pub struct Downing {
  pub ver: Ver,
  pub down: Down,
  pub tar: String,
}

async fn bgu(name: impl AsRef<str>, now_ver: Ver, site: iget::Site) -> Result<Option<Downing>> {
  let name = name.as_ref();
  let ver_txt = site.txt(format!("v/{name}")).await?;
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

impl<'a> Bgu<'a> {
  pub fn new(
    pk: &'a [u8; PUBLIC_KEY_LENGTH],
    name: impl Into<String>,
    now_ver: Ver,
    mirror: &[(bool, impl AsRef<str>)],
  ) -> Self {
    let name = name.into();
    let site = iget::Site::rand_new("https://", mirror);
    Self {
      ing: tokio::spawn(bgu(name.clone(), now_ver, site)),
      name,
      pk,
    }
  }

  pub async fn join(self) -> Result<Option<Ver>> {
    if let Some(ing) = self.ing.await?? {
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
                  let mut bin_dir: PathBuf = XDG_BIN_HOME();
                  bin_dir.push(&self.name);
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
        let verify = VerifyingKey::from_bytes(self.pk)?;
        let sign = Signature::from_bytes(&hsc);
        if xerr::is_ok!(verify.verify(hash.as_bytes(), &sign)) {
          let name = self.name;
          let exe = bin_dir.join(name.clone() + std::env::consts::EXE_SUFFIX);
          use std::process::Command;
          let out = Command::new(exe).args(["-v"]).output()?;
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
              let now_ver = ing.ver;
              if ver > now_ver {
                dbg!(ver);
              } else {
                tracing::error!("{} update ver {ver} <= now {now_ver}", name);
              }
            }
          }
        } else {
          xerr::log!(remove_dir_all(bin_dir).await);
        }
      }
    }
    Ok(None)
  }
}
