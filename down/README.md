# down

```rust
#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

use aok::Result;
use ireq::REQ;

pub async fn meta(url: &str) -> Result<(u64, ireq::reqwest::Url)> {
  let res = REQ
    .get(url)
    .header("User-Agent", "curl/8.4.0")
    .send()
    .await?;
  let status = res.status();
  if ireq::SUCCESS_STATUS.contains(&status) {
    return Ok((res.content_length().unwrap_or(0), res.url().clone()));
  }
  Err(ireq::ReqError::Status(status, res.text().await?).into())
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
