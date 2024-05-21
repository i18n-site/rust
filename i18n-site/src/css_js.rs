use std::path::Path;

use i18::EMPTY;
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use tracing::error;

#[derive(Debug)]
pub struct CssJs {
  pub css: String,
  pub js: String,
}

pub fn css_js(root: &Path, foot_htm: &str) -> CssJs {
  let root = root.join(".i18n/htm");

  CssJs {
    css: {
      let fp = root.join("_.css");

      let mut css = if fp.exists() {
        if let Ok(r) = xerr::ok!(ifs::rtxt(&fp)) {
          r
        } else {
          EMPTY
        }
      } else {
        EMPTY
      };

      if !css.is_empty() {
        if let Ok(mut stylesheet) =
          xerr::ok!(StyleSheet::parse(&css.clone(), ParserOptions::default(),))
        {
          xerr::log!(stylesheet.minify(MinifyOptions::default()));
          if let Ok(t) = stylesheet.to_css(PrinterOptions {
            minify: true,
            ..Default::default()
          }) {
            css = t.code;
          }
        }
      }

      css
    },
    js: {
      let fp = root.join("_.js");

      let mut js = if fp.exists() {
        if let Ok(r) = xerr::ok!(ifs::rtxt(&fp)) {
          r
        } else {
          EMPTY
        }
      } else {
        EMPTY
      };

      if !foot_htm.is_empty() {
        if let Ok(foot_htm) = xerr::ok!(sonic_rs::to_string(foot_htm)) {
          js = format!(
            "export const F=(I)=>`{}`\n{js}",
            &foot_htm[1..foot_htm.len() - 1]
          );
        }
      }

      match minjs::minjs(&js) {
        Ok(s) => js = s,
        Err(err) => {
          error!("{}\n{err}", fp.display());
        }
      }
      js
    },
  }
}
