# xboot

## English Readme

With the help of [linkme](https://github.com/dtolnay/linkme), call an asynchronous function to initialize static variables.

You can connect to the database before the program starts and set it as a module-level variable, refer to [xkv: redis global connector](https://crates.io/crates/xkv).

linkme will report an error when linking with `rust-lld` on Linux [Missing symbol on Linux](https://github.com/dtolnay/linkme/issues/107).

Please use [mold](https://github.com/rui314/mold) instead, and configure the environment variables as follows:

```
export RUSTFLAGS="$RUSTFLAGS -C linker=clang -C link-arg=-fuse-ld=/usr/bin/mold"
```

## 中文说明

借助 [linkme](https://github.com/dtolnay/linkme)，调用异步函数初始化静态变量。

可在程序启动之前连上数据库，并设置为一个模块级别的变量，参考  [xkv: redis 全局连接器](https://crates.io/crates/xkv) 。

linkme 在 Linux 上用 `rust-lld` 链接会报错 [Missing symbol on Linux](https://github.com/dtolnay/linkme/issues/107)

请改用 [mold](https://github.com/rui314/mold) ，配置环境变量如下:

```
export RUSTFLAGS="$RUSTFLAGS -C linker=clang -C link-arg=-fuse-ld=/usr/bin/mold"
```

## 演示代码

```rust
use aok::{Result, OK};
use tokio::time::{sleep, Duration};
use tracing::info;

pub struct Client {}

impl Client {
  pub async fn test(&self) {
    info!("client test success");
  }
}

pub async fn connect() -> Result<Client> {
  info!("Sleeping for 3 seconds...");
  sleep(Duration::from_secs(3)).await;
  Ok(Client {})
}

static_::init!(CLIENT: Client {
  connect().await
});

#[tokio::main]
async fn main() -> Result<()> {
  static_::init().await?;
  info!("inited");
  CLIENT.test().await;
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
