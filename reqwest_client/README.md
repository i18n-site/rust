# reqwest_client

```rust
use std::time::Duration;

use reqwest::{Client, ClientBuilder};

pub fn client() -> ClientBuilder {
  Client::builder()
    // .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
    .user_agent("curl/8.1.1")
    .zstd(true)
    .gzip(true)
    .brotli(true)
    .connect_timeout(Duration::from_secs(9))
}

#[cfg(any(feature = "client", feature = "proxy"))]
pub const TIMEOUT: u64 = 120;

#[cfg(feature = "client")]
#[static_init::dynamic]
pub static CLIENT: Client = client()
  .timeout(Duration::from_secs(TIMEOUT))
  .build()
  .unwrap();

#[cfg(feature = "proxy")]
mod proxy;

#[cfg(feature = "proxy")]
pub use proxy::proxy;

#[cfg(feature = "proxy_from_env")]
mod proxy_from_env;

#[cfg(feature = "proxy_from_env")]
pub use proxy_from_env::{PROXY, ProxyIter, proxy_iter};
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
