pub mod flag;
pub mod flag_li;
pub mod pattern;
pub mod random_pos;
pub mod svg;
pub use svg2webp::svg2webp;

pub use crate::flag_li::{Flag, N};

pub fn gen(width: u32, height: u32) -> Result<(Box<[u8]>, [Flag; N]), svg2webp::Error> {
  let (xml, flag_li) = svg::gen(width, height);
  Ok((svg2webp(xml, 40.0)?, flag_li))
}
