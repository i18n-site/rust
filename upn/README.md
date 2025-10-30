# unicode punctuation / 全角标点

```rust
use upn::is_full_width_break;

#[test]
fn test_full_width_break_characters() {
  // 符合 is_full_width_break 的全角标点
  let full_width_break_chars = vec![
    '’', '”', '…', '、', '。', '〉', '》', '』', '】', '﹑', '！', '）', '，', '．', '：', '；',
    '？', '｝',
  ];

  for &c in &full_width_break_chars {
    assert!(
      is_full_width_break(c),
      "字符 '{}' 应该被识别为全角换行标点",
      c
    );
  }
}

#[test]
fn test_non_full_width_break_characters() {
  // 不符合 is_full_width_break 的非全角标点
  let non_full_width_break_chars = vec!['a', '1', '.', ',', '-', '_', ' ', '(', '[', 'A'];

  for &c in &non_full_width_break_chars {
    assert!(
      !is_full_width_break(c),
      "字符 '{}' 不应被识别为全角换行标点",
      c
    );
  }
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
