#![feature(trait_alias)]

use tran_trait::Parse;
mod toml;
mod yaml;
use aok::Result;
use txt_li::TxtLi;

pub const TRAN: [&str; 5] = ["description", "title", "summary", "brief", "author"];

pub const HUGO_TOML: &str = "+++";
pub const HUGO_YAML: &str = "---";
pub const HUGO_HEAD: [&str; 2] = [HUGO_YAML, HUGO_TOML];
pub const HUGO_YAML_POS: usize = 0;
pub const HUGO_TOML_POS: usize = 1;

pub fn parse<P: Parse, S: Into<String>>(iter: impl IntoIterator<Item = S>) -> Result<TxtLi> {
  let mut txt_li = TxtLi::new();
  let mut iter = iter.into_iter().map(|i| i.into());
  if let Some(first_line) = iter.next() {
    #[allow(clippy::never_loop)]
    'out: loop {
      for (pos, prefix) in HUGO_HEAD.into_iter().enumerate() {
        if first_line == prefix {
          txt_li.push_no_tran_line(prefix);
          let mut buf = vec![];
          for i in iter.by_ref() {
            if i != prefix {
              buf.push(i);
            } else {
              let t = buf.join("\n");
              if pos == HUGO_YAML_POS {
                yaml::parse::<P>(&mut txt_li, t)?;
              } else if pos == HUGO_TOML_POS {
                toml::parse::<P>(&mut txt_li, t)?;
              }
              txt_li.push_no_tran_line(prefix);
              P::parse(&mut txt_li, iter)?;
              break 'out;
            }
          }
          // 没找到闭合标记
          P::parse(&mut txt_li, buf.into_iter())?;
          break 'out;
        }
      }
      // 开头不是 hugo 标记
      P::parse(&mut txt_li, [first_line].into_iter().chain(iter))?;
      break;
    }
  }

  // 找不到闭合, 忽略
  Ok(txt_li)
}
