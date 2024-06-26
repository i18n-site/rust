#![feature(const_trait_impl)]

use std::future::Future;

use aok::Result;
use bgu::PUBLIC_KEY_LENGTH;
use const_str::concat;
pub use const_str::{parse, split};

pub const PK: &[u8; PUBLIC_KEY_LENGTH] = include_bytes!("i18n.pk");

pub const GH: &str = "/gh/i18n-site/dist@";

pub const MIRROR: &[&str] = &[
  "github.com/i18n-site/rust/releases/download/",
  concat!("cdn.jsdelivr.net", GH),
  "xxai.eu.org/",
  concat!("fastly.jsdelivr.net", GH),
  "codeberg.org/i18n-site/dist/raw/branch/",
  concat!("cdn.jsdmirror.com", GH),
  "raw.githubusercontent.com/i18n-site/dist/",
  "bitbucket.org/i18nsite/dist/raw/",
  // atomgit.com/i18n-site/dist/raw/
  // "jsd.onmicrosoft.cn/gh/i18n-site/dist@",
  // "mirror.ghproxy.com/github.com/i18n-site/rust/releases/download/",
  // huggingface.co/datasets/i18n-site/dist/raw/",
];

pub fn boot<F: Future<Output = Result<()>>>(
  name: impl Into<String>,
  ver: [u32; 3],
  main: impl Fn() -> F,
) -> impl Future<Output = Result<()>> {
  bgu::boot(PK, MIRROR, name, ver, main)
}

#[macro_export]
macro_rules! boot {
  ($main:expr) => {{
    $crate::boot(
      env!("CARGO_PKG_NAME"),
      {
        const ver: [&str; 3] = $crate::split!(env!("CARGO_PKG_VERSION"), ".");
        [
          $crate::parse!(ver[0], u32),
          $crate::parse!(ver[1], u32),
          $crate::parse!(ver[2], u32),
        ]
      },
      $main,
    )
  }};
}
