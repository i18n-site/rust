# is_cn_char

```rust
pub fn is_cn_char(i: char) -> bool {
  let i = i as u32;
  for [b, e] in [
    [0x4E00, 0x9FA5],
    [0x9FA6, 0x9FCB],
    [0x3400, 0x4DB5],
    [0x20000, 0x2A6D6],
    [0x2A700, 0x2B734],
    [0x2B740, 0x2B81D],
    [0x2F00, 0x2FD5],
    [0x2E80, 0x2EF3],
    [0xF900, 0xFAD9],
    [0x2F800, 0x2FA1D],
    [0xE815, 0xE86F],
    [0xE400, 0xE5E8],
    [0xE600, 0xE6CF],
    [0x31C0, 0x31E3],
    [0x2FF0, 0x2FFB],
    [0x3105, 0x3120],
    [0x31A0, 0x31BA],
  ] {
    if i >= b && i <= e {
      return true;
    }
  }
  if i == 0x3007 {
    return true;
  }
  false
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
