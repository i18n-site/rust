# hugo_head

```rust
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
