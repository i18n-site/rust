# len_mtime

len mtime 用 db 存

语言文件存

lang hash_len src_hash

```rust
use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

// #[tokio::test]
// async fn test() -> Result<()> {
//   info!("{}", 123456);
//   OK
// }

#[test]
fn test() -> Result<()> {
  let workdir = env!("CARGO_MANIFEST_DIR");
  let workdir: std::path::PathBuf = workdir.into();
  let dbdir = workdir.join("tests/db");
  let mut len_mtime = len_mtime::LenMtime::load(&dbdir, &workdir)?;

  for i in len_mtime.is_change([
    ("Cargo.toml", vec![]),
    // "README.mdt",
    // "src/lib.rs",
    // "tests/main.rs",
  ])? {
    dbg!(i);
  }

  let need_save = ["Cargo.toml"];
  len_mtime.save(need_save)?;
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