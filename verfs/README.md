# verfs

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
  // let dir: PathBuf = env!("CARGO_MANIFEST_DIR").into();
  // let yml = dir.join("tests/test.yml");
  // let mut hash = HashMap::new();
  // let r = latest_ver(&yml, &mut hash)?;
  // dbg!(r);
  // let tests = dir.join("tests");
  //
  // let mut verfs = VerFs::load(&tests, tests.join("out"), tests.join("log"))?;
  //
  // verfs.cp("main.rs")?;
  // verfs.wstr("a/b/c.txt", "123\n456")?;
  // verfs.wstr("ab/b/c.txt", "123\n456")?;
  // verfs.wstr("b/1/2.txt", "123\n")?;
  // verfs.wstr("b/2/2.txt", "123\n456")?;
  // verfs.save()?;
  //
  // let mut prefix_li = PrefixLi::new(vec!["a".into(), "b".into()]);
  //
  // for (k, v) in verfs.sorted_rel_ver() {
  //   info!("{k} {v}");
  //   prefix_li.add(k, v);
  // }
  // for (prefix, i) in prefix_li.0 {
  //   println!("{prefix} {}", to_string(&i)?);
  // }

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