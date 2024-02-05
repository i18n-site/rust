#![feature(const_trait_impl)]
#![feature(effects)]
#![feature(macro_metavar_expr)]

use std::{
  env::temp_dir,
  fmt::{Debug, Display},
  fs::OpenOptions,
  io::{BufReader, BufWriter, Seek},
};

use aok::Result;
use bufstream::BufStream;
pub use const_str;
use current_platform::CURRENT_PLATFORM;
pub use ed25519_dalek::PUBLIC_KEY_LENGTH;
use iget::Down;
use tokio::task::JoinHandle;

#[derive(Default, PartialOrd, Ord, PartialEq, Eq)]
pub struct Ver(pub [u32; 3]);

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

pub const EXT_B3S: &str = ".b3s";

pub struct Downing {
  pub ver: Ver,
  pub down: [Down; 2],
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
  let tar = format!("{name}/{ver_txt}/{CURRENT_PLATFORM}.tar.xz");
  let dir: String = temp_dir().as_os_str().to_string_lossy().into();

  let tar_fp = dir + &tar;

  let (dtar, db3s) = trt::join!(
    site.down(tar.clone() + EXT_B3S, tar_fp.clone() + EXT_B3S),
    site.down(tar, tar_fp.clone())
  );

  Ok(Some(Downing {
    tar: tar_fp,
    ver,
    down: [dtar, db3s],
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
      for i in ing.down {
        i.show().await?;
      }
      let tar = ing.tar.clone();
      let (b3s, hash) = trt::join!(ifs::r(ing.tar + EXT_B3S), ifs::hash(&tar));

      use ed25519_dalek::{Signature, Verifier, VerifyingKey};

      let verify = VerifyingKey::from_bytes(self.pk)?;
      let sign = Signature::from_bytes(&b3s[..].try_into()?);
      match verify.verify(&hash, &sign) {
        Ok(_) => {
          let xz = std::fs::File::open(&tar)?;
          let mut r = BufReader::new(xz);
          dbg!(&tar[..tar.len() - 3]);
          let mut tar = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&tar[..tar.len() - 3])?;

          let mut w = BufWriter::new(tar);
          lzma_rs::xz_decompress(&mut r, &mut w)?;
          let tar = w.get_mut();
          tar.seek(std::io::SeekFrom::Start(0))?;
          let tar = tar::Archive::new(tar);

          return Ok(Some(ing.ver));
        }
        Err(err) => {
          let ver = ing.ver;
          tracing::warn!("{ver} : b3s verify failed {:?}", err);
        }
      }
    }
    Ok(None)
  }
}
