mod error;
pub use error::Error;
use zenwebp::{EncodeRequest, LossyConfig, PixelLayout};

/// # [English]
/// Convert SVG to WebP format.
///
/// - `svg`: SVG string or data.
/// - `quality`: Encoding quality (0 to 100). 0 is the smallest size, 100 is the best quality. Default is usually 75.
///
/// # [中文]
/// 将 SVG 转换为 WebP 格式。
///
/// - `svg`: SVG 字符串或数据。
/// - `quality`: 编码质量 (0 到 100)。0 为最小体积，100 为最佳质量。通常默认值为 75。
pub fn svg2webp(svg: impl AsRef<str>, quality: u8) -> Result<Box<[u8]>, Error> {
  let opt = usvg::Options::default();
  let rtree = usvg::Tree::from_data(svg.as_ref().as_bytes(), &opt)?;

  let pixmap_size = rtree.size();
  let (width, height) = (pixmap_size.width() as u32, pixmap_size.height() as u32);

  let mut pixmap = tiny_skia::Pixmap::new(width, height).ok_or(Error::PixmapNew)?;

  // 设置白色背景 / Set white background
  pixmap.fill(tiny_skia::Color::WHITE);

  resvg::render(&rtree, usvg::Transform::default(), &mut pixmap.as_mut());

  let config = LossyConfig::new().with_quality(quality as f32);
  let webp = EncodeRequest::lossy(&config, pixmap.data(), PixelLayout::Rgba8, width, height)
    .encode()
    .map_err(|_| Error::Encode)?;

  Ok(webp.into_boxed_slice())
}
