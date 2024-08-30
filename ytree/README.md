# ytree

```rust
use std::io::{BufRead, Cursor};

use aok::{Result, OK};
use lang::Lang;
use static_init::constructor;
use tracing::info;
use ytree::{sitemap::lang_li_e, Li};

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let paths = [
    "README.md#a",
    "blog/README.md#2",
    "blog/news/README.md#c",
    "blog/news/begin.md#d",
    "x/news/1.md#x",
    "x/2/3.md#y",
  ];

  let mut root = Li(Vec::new());

  for path in paths {
    root.push(path);
  }

  // root.remove(paths[1]);
  // root.remove(paths[0]);

  for i in root.iter() {
    info!("{i}");
  }

  // let yml = serde_yaml::to_string(&root).unwrap();
  // info!("{}", yml);

  let mut bitmap = roaring::RoaringBitmap::new();
  for i in [Lang::Ja, Lang::En, Lang::Zh, Lang::ZhTw] {
    bitmap.insert(i as u32);
  }

  let yml = ytree::sitemap::dumps([(lang_li_e(&bitmap), root)]);

  info!("{yml}");
  let cursor = Cursor::new(yml.as_bytes());

  let yml = ytree::sitemap::loads(cursor.lines().map_while(Result::ok));

  let t = yml.sitemap("/Users/z/i18n/md")?;
  for i in &t.rel_lang_set {
    info!("{} {:?}", i.0, i.1);
  }
  // for i in t.set() {
  //   println!("{i}");
  // }
  info!("{}", t.dumps());
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