use image_webp::{ColorType, WebPEncoder};
use thiserror::Error;
use tiny_skia::PremultipliedColorU8;

#[derive(Error, Debug)]
pub enum Error {
  #[error("tiny_skia::Pixmap::new return None")]
  Pixmap,

  #[error("resvg::render return None")]
  ReSvg,

  #[error("encode error")]
  Encode,
}

pub fn svg2webp(
  svg: impl AsRef<str>,
  // quality: f32
) -> Result<Box<[u8]>, Error> {
  let opt = usvg::Options::default();
  // let fontdb = usvg::fontdb::Database::new();, &fontdb
  if let Ok(rtree) = usvg::Tree::from_data(svg.as_ref().as_bytes(), &opt) {
    let pixmap_size = rtree.size();
    let width = pixmap_size.width() as u32;
    let height = pixmap_size.height() as u32;
    if let Some(mut pixmap) = tiny_skia::Pixmap::new(width, height) {
      // 去除透明度（默认是黑底，255-颜色会改为用白底）
      for px in pixmap.pixels_mut() {
        *px =
          PremultipliedColorU8::from_rgba(255 - px.red(), 255 - px.green(), 255 - px.blue(), 255)
            .unwrap();
      }
      resvg::render(&rtree, usvg::Transform::default(), &mut pixmap.as_mut());
      let img = pixmap.data();

      let mut webp = Vec::new();
      let encoder = WebPEncoder::new(&mut webp);
      if encoder
        .encode(img, width, height, ColorType::Rgba8)
        .is_err()
      {
        return Err(Error::Encode);
      }
      return Ok(webp.into());
    }
  } else {
    return Err(Error::ReSvg);
  }
  Err(Error::Pixmap)?
}
