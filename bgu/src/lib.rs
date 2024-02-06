#![feature(const_trait_impl)]
#![feature(effects)]
#![feature(macro_metavar_expr)]

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
use tokio::task::JoinHandle;

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
      pk,
      ing: tokio::spawn(bgu(name, now_ver, site)),
    }
  }

  pub async fn join(self) -> Result<Option<Ver>> {
    if let Some(ing) = self.ing.await?? {
      use std::{fs::File, io::BufReader};

      use ed25519_dalek::{Signature, VerifyingKey};
      // use ed25519_dalek::{Signature, Verifier, VerifyingKey};
      let _verify = VerifyingKey::from_bytes(self.pk)?;

      ing.down.show().await?;

      let mut b3s = [0u8; SIGNATURE_LENGTH];
      // let mut txz = None;

      for entry in ifs::txz::Tar::new(BufReader::new(File::open(&ing.tar)?)).entries()? {
        let mut entry = entry?;
        if let Ok(path) = entry.path() {
          if let Some(ext) = path.extension() {
            if let Some(ext) = ext.to_str() {
              match ext {
                "b3s" => {
                  entry.read(&mut b3s[..])?;
                }
                "txz" => {}
                _ => {}
              }
            }
          }
        }
      }

      let _sign = Signature::from_bytes(&b3s);

      // let tar = ing.tar.clone();
      // let b3s_fp = ing.tar + EXT_B3S;
      // let (b3s, hash) = trt::join!(ifs::r(&b3s_fp), ifs::hash(&tar));

      // let b3s = match b3s[..].try_into() {
      //   Ok(r) => r,
      //   Err(_) => {
      //     tracing::warn!("b3s length {} != 64 {b3s_fp}", b3s.len());
      //     let _ = remove_file(b3s_fp).await;
      //     return Ok(None);
      //   }
      // };
      //
      // match verify.verify(&hash, &sign) {
      //   Ok(_) => {
      //     let mut bin_dir: PathBuf = XDG_BIN_HOME();
      //     let t = Into::<PathBuf>::into(&tar[..tar.len() - 4]);
      //     let bin = t.iter().rev().take(3).collect::<Vec<_>>();
      //     let bin_name = bin.last().unwrap().to_string_lossy() + EXE_SUFFIX;
      //     bin.into_iter().rev().for_each(|i| bin_dir.push(i));
      //     let mut exe = bin_dir.clone();
      //     exe.push(bin_name.as_ref());
      //     spawn_blocking(move || ifs::txz::d(&tar, bin_dir)).await??;
      //     dbg!(exe);
      //     return Ok(Some(ing.ver));
      //   }
      //   Err(err) => {
      //     let ver = ing.ver;
      //     tracing::warn!("{ver} : b3s verify failed {:?}", err);
      //     let _ = remove_file(b3s_fp).await;
      //     let _ = remove_file(tar).await;
      //   }
      // };
    }
    Ok(None)
  }
}
