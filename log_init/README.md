# log_init

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]

mod kv;
pub mod layout;

#[cfg(all(unix, feature = "systemd"))]
use std::env;

use kv::Kv;
use logforth::{append, filter::env_filter::EnvFilterBuilder};

#[static_init::dynamic]
pub static TZ: jiff::tz::TimeZone = jiff::tz::TimeZone::try_system().unwrap();

pub fn init() {
  // Check if we're in a systemd environment (unix systems with INVOCATION_ID)
  #[cfg(all(unix, feature = "systemd"))]
  if env::var("INVOCATION_ID").is_ok() {
    // journald is inherently unbuffered (uses UnixDatagram which doesn't buffer)
    if let Ok(journald) = logforth_append_journald::Journald::new() {
      logforth::starter_log::builder()
        .dispatch(|d| {
          d.filter(EnvFilterBuilder::from_default_env().build())
            .append(journald)
        })
        .apply();
      return;
    }
  }
    // If journald fails, fall back to stdout

  // Fallback to stdout logging
  #[cfg(feature = "stdout")]
  {
    let stdout = append::Stdout::default().with_layout(layout::Text::default());

    logforth::starter_log::builder()
      .dispatch(|d| {
        d.filter(EnvFilterBuilder::from_default_env().build())
          .append(stdout)
      })
      .apply();
  }
  #[cfg(not(feature = "stdout"))]
  {
    panic!("No logging backend available")
  }
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

* [i18 : MarkDown 命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖 （ 如果这段原文没有被修改 ）。

* [i18n.site : MarkDown 多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。
