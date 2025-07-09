# bjs

```rust
use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  // let root = std::env!("CARGO_MANIFEST_DIR");
  //
  // let root = format!("{root}/tests");
  // let ctx = &mut bjs::ctx(&root, &root);
  // let mut map1 = HashMap::new();
  // map1.insert("key1".to_string(), "value1".to_string());
  // map1.insert("key2".to_string(), "value2".to_string());
  //
  // let mut map2 = HashMap::new();
  // map2.insert("key3".to_string(), "value3".to_string());
  // map2.insert("key4".to_string(), "value4".to_string());
  //
  // let arg = [
  //   bjs::li_hashmap_to_jsvalue(ctx, &[map1, map2]),
  //   bjs::li_str_to_jsvalue(ctx, &["x", "b"]),
  // ];
  //
  // match bjs::default(ctx, format!("{root}/test.js"), &arg) {
  //   Ok(r) => {
  //     // dbg!(bjs::VecOrStr::parse(r, ctx));
  //     let r = bjs::obj2map(r).unwrap();
  //     dbg!(r);
  //     // dbg!(&r);
  //     // if let Some(file) = r.remove("file") {
  //     //   dbg!(bjs::li_str(ctx, file));
  //     // }
  //   }
  //   Err(err) => {
  //     info!("{}", err);
  //   }
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