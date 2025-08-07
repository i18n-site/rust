# s3_put

backblaze 必须用 " 应用程序密钥 " 而不是 " 主密钥 ", 不然[会报错 "Malformed Access Key Id"](https://github.com/timmyomahony/craft-remote-backup/issues/11#issuecomment-657661478)

```rust
#![feature(async_closure)]
#![feature(let_chains)]

use std::sync::Arc;

use aok::{Result, OK};
use map_await::{MapAwait, StreamExt};
use s3_put::S3Bucket;
use static_init::constructor;
use tracing::info;
use walkdir::WalkDir;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  let dir = env!("CARGO_MANIFEST_DIR");
  let toml = format!("{dir}/Cargo.toml");
  info!("{}", toml);
  let s3 = Arc::new(S3Bucket::from_env("i18ntmp"));
  let mut iter = WalkDir::new(dir).map_unordered(3, move |i| {
    let s3 = s3.clone();
    async move {
      if let Ok::<walkdir::DirEntry, _>(i) = i {
        let file_type = i.file_type();
        if file_type.is_file() {
          let path = i.path();

          if let Some(url) = path.strip_prefix(dir)?.as_os_str().to_str() {
            info!("begin upload {url}");
            s3.put(url, "text/js", path).await?;
            info!("uploaded {url}");
          }
        }
      }
      OK
    }
  });
  while let Some(r) = iter.next().await {
    r?;
  }
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