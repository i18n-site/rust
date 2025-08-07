# cookie_expire

```rust
use chrono::{DateTime, Utc};
use coarsetime::Clock;

pub fn cookie_expire_with_days(days: u64) -> String {
  let now = Clock::now_since_epoch();
  let future_time = now + coarsetime::Duration::from_secs(days * 24 * 60 * 60);
  let datetime = DateTime::<Utc>::from_timestamp(future_time.as_secs() as i64, 0).unwrap();
  datetime.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}

pub fn cookie_expire() -> String {
  // 从 Chrome M104（2022 年 8 月）开始，Cookie 所设置的有效期不能超过 400 天。 https://developer.chrome.com/blog/cookie-max-age-expires
  cookie_expire_with_days(400)
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
