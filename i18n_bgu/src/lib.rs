#![feature(const_trait_impl)]
#![feature(effects)]

use std::future::Future;

use aok::Result;
use bgu::PUBLIC_KEY_LENGTH;
pub use const_str::{parse, split};

pub const PK: &[u8; PUBLIC_KEY_LENGTH] = include_bytes!("i18n.pk");

pub const MIRROR: &[&str] = &[
  // atomgit.com/i18n-site/dist/raw/
  // "jsd.onmicrosoft.cn/gh/i18n-site/dist@",
  "cdn.jsdelivr.net/gh/i18n-site/dist@",
  // "mirror.ghproxy.com/github.com/i18n-site/rust/releases/download/",
  "fastly.jsdelivr.net/gh/i18n-site/dist@",
  "codeberg.org/i18n-site/dist/raw/branch/",
  "github.com/i18n-site/rust/releases/download/",
  "cdn.jsdmirror.com/gh/i18n-site/dist@",
  "raw.githubusercontent.com/i18n-site/dist/",
  "xxai.eu.org/",
  "bitbucket.org/i18nsite/dist/raw/",
  // (false, "huggingface.co/datasets/i18n-site/dist/raw/"),
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
