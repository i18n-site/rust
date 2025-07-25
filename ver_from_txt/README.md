# ver_from_txt

```rust
#![feature(slice_concat_trait)]

use std::{error::Error, fmt};

use tracing::{error, warn};
use sver::Ver;
use base64::{Engine, engine::general_purpose::STANDARD};
use aok::Result;
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
```

## About

This project is an open-source component of [i18n.site ⋅ Internationalization Solution](https://i18n.site).

* [i18 : MarkDown Command Line Translation Tool](https://i18n.site/i18)

  The translation perfectly maintains the Markdown format.

  It recognizes file changes and only translates the modified files.

  The translated Markdown content is editable; if you modify the original text and translate it again, manually edited translations will not be overwritten (as long as the original text has not been changed).

* [i18n.site : MarkDown Multi-language Static Site Generator](https://i18n.site/i18n.site)

  Optimized for a better reading experience

## 关于

本项目为 [i18n.site ⋅ 国际化解决方案](https://i18n.site) 的开源组件。

* [i18 :  MarkDown命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖（如果这段原文没有被修改）。

* [i18n.site : MarkDown多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。
