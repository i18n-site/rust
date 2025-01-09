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
use svg2avif::svg2avif;

#[cfg(feature = "make")]
pub use crate::ico::{IcoPosLi, PosLi};

#[cfg(feature = "make")]
pub fn make<S: AsRef<str>>(
  width: u32,
  height: u32,
  ico_li: impl AsRef<[S]>,
) -> aok::Result<(Box<[u8]>, IcoPosLi)> {
  let (xml, ico) = svg::make(width, height, ico_li);
  Ok((svg2avif(xml, 30.0, 10)?, ico))
}

#[cfg(feature = "verify")]
fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
  ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

#[cfg(feature = "verify")]
pub fn verify(pos_li: PosLi, click_x_y: &[u64]) -> bool {
  if click_x_y.len() < 2 * pos_li.len() {
    return false;
  }

  for (pos, xys) in pos_li.iter().enumerate() {
    let pos = pos * 2;
    let cx = (click_x_y[pos]) as f32;
    let cy = (click_x_y[pos + 1]) as f32;

    // 起点 + 半径 = 圆心
    let size = (xys.size as f32) / 2.0;

    if distance(xys.x as f32 + size, xys.y as f32 + size, cx, cy) > size {
      return false;
    }
  }
  true
}
