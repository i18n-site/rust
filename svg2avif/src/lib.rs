use aok::Result;
use ravif::{Encoder, Img, RGBA8};
use thiserror::Error;
use tiny_skia::PremultipliedColorU8;

#[derive(Error, Debug)]
pub enum Error {
  #[error("tiny_skia::Pixmap::new return None")]
  PIXMAP,

  #[error("resvg::render return None")]
  RESVG,
}

pub fn svg2avif(svg: impl AsRef<str>, quality: f32, speed: u8) -> Result<Box<[u8]>> {
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
      let img: &[RGBA8] = bytemuck::cast_slice(img);
      let img = Img::new(img, width as _, height as _);
      let avif = Encoder::new()
        .with_quality(quality)
        .with_speed(speed)
        .encode_rgba(img)?
        .avif_file;

      // let encoder = Encoder::from_rgba(img, width, height);
      // let encoded_avif = encoder.encode(quality);
      return Ok(avif.into());
    }
  } else {
    Err(Error::RESVG)?;
  }
  Err(Error::PIXMAP)?
}
