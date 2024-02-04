#![feature(const_trait_impl)]
#![feature(effects)]

use std::fmt::Debug;

use aok::Result;
pub use const_str;

#[derive(Default, PartialOrd, Ord, PartialEq, Eq)]
pub struct Ver(pub u32, pub u32, pub u32);

impl Debug for Ver {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Ver({}.{}.{})", self.0, self.1, self.2)
  }
}

#[macro_export]
macro_rules! ver {
  () => {{
    use $crate::const_str::{parse, split};
    let r: [&str; 3] = split!(env!("CARGO_PKG_VERSION"), ".");
    $crate::Ver(parse!(r[0], u32), parse!(r[1], u32), parse!(r[2], u32))
  }};
}

impl TryFrom<&[u8]> for Ver {
  type Error = vb::Error;

  fn try_from(bin: &[u8]) -> Result<Self, Self::Error> {
    let v = vb::d(bin)?;
    let len = v.len();
    if v.len() != 3 {
      tracing::warn!("ver.len != 3 > {:?}", bin);
    }
    // major.minor.patch
    let major = if len > 0 { v[0] as _ } else { 0 };
    let minor = if len > 1 { v[1] as _ } else { 0 };
    let patch = if len > 2 { v[2] as _ } else { 0 };
    Ok(Ver(major, minor, patch))
  }
}

#[derive(Debug, Default)]
pub struct ChangeLog {
  pub ver: Ver,
  pub txt: String,
}

pub async fn bgu(ver: Ver, mirror: &[(bool, impl AsRef<str>)]) -> Result<Vec<ChangeLog>> {
  dbg!(ver);
  let site = iget::Site::rand_new("https://", mirror);
  let log = ChangeLog::default();
  let v: Ver = (&site.bin("_/v").await?[..]).try_into()?;
  dbg!(v);
  dbg!(&log);
  dbg!(&site);
  Ok(vec![])
}
