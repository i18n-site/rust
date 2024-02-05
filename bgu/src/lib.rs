#![feature(const_trait_impl)]
#![feature(effects)]
#![feature(macro_metavar_expr)]

use std::{env::temp_dir, fmt::Debug};

use aok::Result;
pub use const_str;
use current_platform::CURRENT_PLATFORM;

#[derive(Default, PartialOrd, Ord, PartialEq, Eq)]
pub struct Ver(pub [u32; 3]);

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

pub async fn bgu(
  name: impl AsRef<str>,
  now_ver: Ver,
  mirror: &[(bool, impl AsRef<str>)],
) -> Result<Option<Ver>> {
  let name = name.as_ref();
  let site = iget::Site::rand_new("https://", mirror);
  let ver = site.txt(format!("v/{name}")).await?;
  let mut ver = ver.as_str();
  if let Some(p) = ver.find('\n') {
    ver = &ver[..p];
  }
  let v: Ver = ver.into();
  if now_ver >= v {
    return Ok(None);
  }
  let prefix = format!("{name}/{ver}/{CURRENT_PLATFORM}.");
  let dir: String = temp_dir().as_os_str().to_string_lossy().into();

  macro_rules! down {
    ($ext:expr) => {{
      let ext = $ext;
      let f = format!("{prefix}{ext}");
      let to = dir.clone() + &f;
      site.down(f, to)
    }};
  }

  let (_tar, _b3s) = trt::join!(down!("tar.xz"), down!("b3s"));

  // dbg!((tar, b3s));
  // let bar = site.down(&url).await?;
  Ok(Some(v))
}
