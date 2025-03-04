# set_cookie

```rust
use http::HeaderValue;

pub const SET_COOKIE: &str = "Set-Cookie";

const MAX: u32 = 99999999;

pub struct Cookie {
  pub domain: String,
}

pub fn new(domain: impl Into<String>) -> Cookie {
  Cookie {
    domain: domain.into(),
  }
}

impl Cookie {
  pub fn set(&self, key: impl AsRef<str>, val: impl AsRef<str>, max_age: u32) -> HeaderValue {
    let key = key.as_ref();
    let val = val.as_ref();
    let cookie = format!(
      "{key}={val};Max-Age={max_age};Domain={};Secure;HttpOnly;Path=/;Partitioned",
      self.domain
    );
    cookie.parse().unwrap()
  }

  pub fn set_max(&self, key: impl AsRef<str>, val: impl AsRef<str>) -> HeaderValue {
    self.set(key, val, MAX)
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

* [i18 :  MarkDown命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖（如果这段原文没有被修改）。

* [i18n.site : MarkDown多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。
