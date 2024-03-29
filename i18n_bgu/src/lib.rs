#![feature(const_trait_impl)]
#![feature(effects)]

use std::future::Future;

use aok::Result;
use bgu::PUBLIC_KEY_LENGTH;

pub const PK: &[u8; PUBLIC_KEY_LENGTH] = include_bytes!("i18n.pk");

pub const MIRROR: &[(bool, &str)] = &[
  (false, "atomgit.com/i18n-site/dist/raw/"),
  (false, "github.com/i18n-site/rust/releases/download/"),
  (false, "codeberg.org/i18n-site/dist/raw/branch/"),
  (false, "raw.githubusercontent.com/i18n-site/dist/"),
  (false, "bitbucket.org/i18nsite/dist/raw/"),
  (true, "xxai.eu.org/"),
  // (false, "huggingface.co/datasets/i18n-site/dist/raw/"),
];

pub static NAME: &str = env!("CARGO_PKG_NAME");

pub fn boot<F: Future<Output = Result<()>>>(
  main: impl Fn() -> F,
) -> impl Future<Output = Result<()>> {
  bgu::boot(PK, MIRROR, NAME, main)
}
