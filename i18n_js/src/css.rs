use std::path::{Path, PathBuf};

use aok::Result;
use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
use tracing::error;

// let hash = burl::e(b3(&css));
pub fn css(root: &Path, css_li: &[PathBuf]) -> Result<String> {
  let mut li = Vec::new();

  for fp in css_li {
    let path = root.join(fp);
    let css = ifs::rstr(&path)?;
    match StyleSheet::parse(&css, ParserOptions::default()) {
      Ok(stylesheet) => {
        match stylesheet.to_css(PrinterOptions {
          minify: true,
          ..Default::default()
        }) {
          Ok(t) => {
            let css = t.code;
            let name = fp.file_name().unwrap().to_str().unwrap();
            if name == "import.css" {
              li.insert(0, css);
            } else {
              li.push(css);
            }
          }
          Err(e) => {
            error!("❌ {:?} {e}", path);
          }
        }
      }
      Err(e) => {
        error!("❌ {:?} {e}", path);
      }
    };
  }

  Ok(li.join(""))
}
