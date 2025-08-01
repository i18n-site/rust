#![feature(const_trait_impl)]

use std::future::Future;

use aok::Result;
use bgu::PUBLIC_KEY_LENGTH;
use const_str::concat;
pub use const_str::{parse, split};

pub const PK: &[u8; PUBLIC_KEY_LENGTH] = include_bytes!("i18n.pk");

pub const GH: &str = "/gh/i18n-site/dist@";

pub const V_HOST: &[&str] = &[
  "github.com/i18n-site/rust/releases/download",
  "i18n-dist.s3.bitiful.net",
  "xxai.eu.org",
  "dist-v.i18n.site",
  "raw.githubusercontent.com/i18n-site/dist",
  "dist18.s3.eu-central-003.backblazeb2.com",
];

pub const MIRROR: &[&str] = &[
  "github.com/i18n-site/rust/releases/download/",
  concat!("cdn.jsdelivr.net", GH),
  "xxai.eu.org/",
  concat!("fastly.jsdelivr.net", GH),
  concat!("cdn.jsdmirror.com", GH),
  "raw.githubusercontent.com/i18n-site/dist/",
  // "bitbucket.org/i18nsite/dist/raw/",
  // "codeberg.org/i18n-site/dist/raw/branch/",
  // atomgit.com/i18n-site/dist/raw/
  // "jsd.onmicrosoft.cn/gh/i18n-site/dist@",
  // "mirror.ghproxy.com/github.com/i18n-site/rust/releases/download/",
  // huggingface.co/datasets/i18n-site/dist/raw/",
];

pub fn boot<F: Future<Output = Result<()>>>(
  name: impl Into<String>,
  ver: [u64; 3],
  main: impl Fn() -> F,
) -> impl Future<Output = Result<()>> {
  bgu::boot(PK, V_HOST, MIRROR, name, ver, main)
}

#[macro_export]
macro_rules! boot {
  ($name:expr,$main:expr) => {{
    $crate::boot(
      $name,
      {
        const ver: [&str; 3] = $crate::split!(env!("CARGO_PKG_VERSION"), ".");
        [
          $crate::parse!(ver[0], u64),
          $crate::parse!(ver[1], u64),
          $crate::parse!(ver[2], u64),
        ]
      },
      $main,
    )
  }};
  ($main:expr) => {{
    $crate::boot!(env!("CARGO_PKG_NAME"), $main)
  }};
}
