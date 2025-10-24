#![cfg_attr(docsrs, feature(doc_cfg))]

/// 需要点击的图片个数
pub const N: usize = 3;

mod pos_li;
pub use pos_li::{Pos, PosLi};

#[cfg(feature = "ico_li")]
mod ico_li;

#[cfg(feature = "ico_li")]
pub use ico_li::ICO_LI;

#[cfg(feature = "wasm")]
mod wasm;

#[cfg(feature = "wasm")]
pub use wasm::Gen;

#[cfg(feature = "make")]
mod ico;
#[cfg(feature = "make")]
mod pattern;
#[cfg(feature = "make")]
mod random_pos;
#[cfg(feature = "make")]
pub mod svg;

#[cfg(feature = "make")]
pub use crate::ico::IcoPosLi;

#[cfg(feature = "verify")]
mod verify;

#[cfg(feature = "verify")]
pub use verify::verify;

#[cfg(feature = "avif")]
pub fn avif<S: AsRef<str>>(
  width: u32,
  height: u32,
  ico_li: impl AsRef<[S]>,
) -> aok::Result<(Box<[u8]>, IcoPosLi)> {
  let (xml, ico) = svg::make(width, height, ico_li);
  Ok((svg2avif::svg2avif(xml, 30.0, 10)?, ico))
}

#[cfg(feature = "webp")]
pub fn webp<S: AsRef<str>>(
  width: u32,
  height: u32,
  ico_li: impl AsRef<[S]>,
) -> aok::Result<(Box<[u8]>, IcoPosLi)> {
  let (xml, ico) = svg::make(width, height, ico_li);
  Ok((svg2webp::svg2webp(xml)?, ico))
}
