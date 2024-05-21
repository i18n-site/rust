pub const TEXT_JS: &str = "text/js";

pub fn mime(fp: &str) -> &'static str {
  if let Some(pos) = fp.rfind('.') {
    let ext = &fp[pos + 1..];
    if ["yml", "json", "md", "txt", "c", "cpp", "rs"].contains(&ext) {
      // for compression : https://developers.cloudflare.com/speed/optimization/content/brotli/content-compression/
      TEXT_JS
    } else {
      mime_guess::from_ext(ext).first_raw().unwrap_or(TEXT_JS)
    }
  } else {
    TEXT_JS
  }
}
