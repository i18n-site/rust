# down

down from multiple URLs to a single file

```rust
use std::path::PathBuf;

use aok::{OK, Void};
use down::down;
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[tokio::test]
async fn test_async() -> Void {
  let file = "0.1.51/x86_64-unknown-linux-musl.tar";
  let tmp: PathBuf = format!("/tmp/{file}").into();
  std::fs::create_dir_all(tmp.parent().unwrap())?;

  let recv = down(
    [
      "github.com/up51/v/releases/download/i18-",
      "up0.u-01.eu.org/i18/",
      "up2.u-01.eu.org/i18/",
      "up3.u-01.eu.org/i18/",
      "yutk.eu.org/i18/",
    ]
    .map(|i| format!("https://{i}{file}")),
    &tmp,
  )
  .await?;
  if let Ok(size) = xerr::ok!(recv.recv().await) {
    while let Ok(info) = recv.recv().await {
      info!("{info}/{size}");
    }
  }

  info!("✅ {}", tmp.display().to_string());
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
