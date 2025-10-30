# cget : macro for clip get_one easy

```rust
use std::process::exit;

use aok::{Result, OK};
use cget::cget;
use clap::{arg, crate_version, Command};
//use current_platform::CURRENT_PLATFORM;

fn main() -> Result<()> {
  let m = Command::new("hsc")
    .disable_version_flag(true)
    .arg(arg!(-c --create "create key if not exist"))
    .arg(arg!(-k --key <key> "key file path"))
    .arg(arg!(-v - -version))
    .arg(arg!(
        - -vv "more version info"
    ))
    .arg(arg!(<fp> "file path"))
    .get_matches();

  if m.get_one("version") == Some(&true) {
    println!(crate_version!());
    exit(0);
  }

  //   if m.get_one("vv") == Some(&true) {
  //     println!(
  //       r#"ver:{}
  // build_target:{}"#,
  //       crate_version!(),
  //       CURRENT_PLATFORM
  //     );
  //     exit(0);
  //   }

  cget!(
    m:
      fp: String;
      key: String;
      create: bool;
  );

  println!("{} {} {}", fp, key, create);

  OK
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