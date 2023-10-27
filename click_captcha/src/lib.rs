mod flag;
mod flag_li;
mod pattern;
mod random_pos;
pub mod svg;
pub use flag::FLAG;
use svg2webp::svg2webp;

pub use crate::flag_li::{Flag, N};

pub fn gen(width: u32, height: u32) -> Result<(Box<[u8]>, [Flag; N]), svg2webp::Error> {
  let (xml, flag_li) = svg::gen(width, height);
  Ok((svg2webp(xml, 40.0)?, flag_li))
}

fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
  ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

pub fn verify(flag_x_y_size: &[u64], click_x_y: &[u64], scale: u64) -> bool {
  if (click_x_y.len() * 3) != (2 * flag_x_y_size.len()) {
    return false;
  }

  for (pos, xys) in flag_x_y_size.chunks(3).enumerate() {
    let pos = pos * 2;
    let cx = (click_x_y[pos] * scale) as f32;
    let cy = (click_x_y[pos + 1] * scale) as f32;
    let size = (xys[2] as f32) / 2.0;
    if distance(xys[0] as f32 + size, xys[1] as f32 + size, cx, cy) > size {
      return false;
    }
  }
  true
}
