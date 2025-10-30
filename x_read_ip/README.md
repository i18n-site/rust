# x_read_ip

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]
#![feature(doc_cfg)]

use std::{borrow::Borrow, net::IpAddr};

use http::HeaderMap;

pub fn get(headers: impl Borrow<HeaderMap>) -> Vec<u8> {
  let header_candidates = ["x-forwarded-for", "x-real-ip", "cf-connecting-ip"];

  for header_name in header_candidates {
    if let Some(header_value) = headers.borrow().get(header_name)
      && let Ok(raw_str) = header_value.to_str()
    {
      let ip_str = raw_str.split(',').next().unwrap_or("").trim();

      if let Ok(ip) = ip_str.parse::<IpAddr>() {
        return match ip {
          IpAddr::V4(ipv4) => ipv4.octets().to_vec(),
          IpAddr::V6(ipv6) => ipv6.octets().to_vec(),
        };
      }
    }
  }

  vec![]
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
