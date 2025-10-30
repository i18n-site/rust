# str0

```rust
use roaring::RoaringTreemap;
use static_init::constructor;
use str0::split;

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
fn test() {
  let li = ["Hello", "skip1", "skip2", "World", "skip3", "!!"];
  let skip_li = RoaringTreemap::from_iter([1, 2, 4]);

  let merged = str0::merge(li, &skip_li);
  assert_eq!(
    merged,
    vec![b'H', b'e', b'l', b'l', b'o', 0, 0, b'W', b'o', b'r', b'l', b'd', 0, b'!', b'!']
  );
  let split_result = split(merged);

  assert_eq!(split_result, ["Hello", "", "", "World", "", "!!"]);
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