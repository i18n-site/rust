#![feature(slice_concat_trait)]

use std::{error::Error, fmt};

use aok::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
use sver::Ver;
use tracing::{error, warn};
mod name_li;
pub use name_li::name_li;

#[derive(Debug)]
pub struct TxtInvalid;

impl fmt::Display for TxtInvalid {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Invalid text was provided")
  }
}

impl Error for TxtInvalid {}

#[derive(Debug)]
pub struct VerUrlLi {
  pub ver: Ver,
  pub url_li: Vec<String>,
}

pub fn ver_from_txt(project: &str, pre_ver: &[u64; 3], txt: &str) -> Result<Option<VerUrlLi>> {
  if let Some((ver, txt)) = txt.split_once(";") {
    let ver = vb::d(STANDARD.decode(ver)?)?;
    if let Ok::<[u64; 3], _>(sver) = ver.try_into() {
      if *pre_ver >= sver {
        return Ok(None);
      }
      let sver = Ver(sver);
      let ver = sver.to_string();
      let mut url_li = vec![];

      for i in txt.split(";") {
        if let Some(first) = i.chars().next()
          && first.is_ascii_uppercase()
        {
          let i = &i[1..];
          match first {
            'G' => {
              let url = format!("https://github.com/{i}/releases/download/{project}-{ver}",);
              // url_li.push(format!("https://github.akams.cn/{url}"));
              url_li.push(url);
            }
            // 不支持断点续传，不用sourceforge
            // 'S' => {
            //   url_li.push(format!(
            //     "https://downloads.sourceforge.net/project/{i}/{project}-{ver}"
            //   ));
            // }
            _ => {
              warn!("txt unknown : {}", i);
            }
          }
          continue;
        } else {
          let suffix = format!("/{project}/{ver}");

          if let Some((prefix, remain)) = i.split_once("[") {
            if let Some((range, remain)) = remain.split_once("]") {
              for i in name_li(range) {
                url_li.push(format!("https://{prefix}{i}{remain}{suffix}",));
              }
            } else {
              error!("txt invalid : {i}");
            }
          } else {
            url_li.push(format!("https://{i}{suffix}"));
          }
        }
      }

      if !url_li.is_empty() {
        return Ok(Some(VerUrlLi { ver: sver, url_li }));
      }
    }
  }

  Err(TxtInvalid.into())
}
