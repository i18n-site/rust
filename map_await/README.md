# map_unordered

```rust
use aok::{Result, OK};
use map_await::MapAwait;
use rand::{rngs::StdRng, Rng, SeedableRng};
use static_init::constructor;
use tokio_stream::StreamExt;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  let mut rng = StdRng::from_rng(&mut rand::rng());
  let end = rng.random_range(6..8);
  for range in [0..0, 0..1, 0..4, 0..end] {
    let mut iter = range.clone().map_unordered(3, |i| async move {
      let mut rng = StdRng::from_rng(&mut rand::rng());
      let sleep = rng.random_range(1000..2000);
      let i = i + 1;
      info!("{i} begin sleep {}", sleep);
      tokio::time::sleep(std::time::Duration::from_millis(sleep)).await;
      info!("{i} done");
      i
    });
    while let Some(i) = iter.next().await {
      info!("{:?}> {i}", &range);
    }
    info!("------");
  }
  info!("exit");
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