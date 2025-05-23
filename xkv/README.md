# xkv

use global static vars connect redis / [kvrocks](https://kvrocks.apache.org) with [fred](https://crates.io/crates/fred)

使用全局静态变量通过 [fred](https://crates.io/crates/fred) 连接 redis / [kvrocks](https://kvrocks.apache.org)。


```rust

use aok::{Result, OK};
use xkv::{
  fred::interfaces::KeysInterface,
  tracing::info,
  R,
};

async fn test_redis() -> Result<()> {
  let key = "xkvtest1";
  let val = "abc";
  R!(del key);

  let v: bool = R.exists(key).await?;
  assert!(!v);

  let v: Option<String> = R.get(key).await?;
  info!("get {key} = {:?}", v);
  assert_eq!(v, None);

  R!(set key, val, None, None, false);

  let v: Option<String> = R.get(key).await?;
  info!("get {key} = {:?}", v);
  assert_eq!(v, Some(val.into()));

  R!(del key);

  OK
}

#[tokio::main]
async fn main() -> Result<()> {
  // 仅在程序启动的main函数中调用一次 / Call it only once in the main function of the program
  static_::init().await?;
  test_redis().await?;
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
