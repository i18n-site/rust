# pos_lines

```rust
use pos_lines::PosLines;

#[test]
fn test_empty_text() {
  let text = "";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![]);
}

#[test]
fn test_single_line() {
  let text = "Hello World";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![(0, "Hello World")]);
}

#[test]
fn test_multiple_lines() {
  let text = "Line 1\nLine 2\nLine 3";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![(0, "Line 1"), (7, "Line 2"), (14, "Line 3"),]);
}

#[test]
fn test_empty_lines() {
  let text = "Line 1\n\nLine 3";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![(0, "Line 1"), (8, "Line 3"),]);
}

#[test]
fn test_mixed_line_endings() {
  let text = "Line 1\r\nLine 2\rLine 3\nLine 4";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(
    lines,
    vec![(0, "Line 1"), (8, "Line 2"), (15, "Line 3"), (22, "Line 4"),]
  );
}

#[test]
fn test_consecutive_line_endings() {
  let text = "Line 1\r\r\nLine 2\n\n\nLine 3";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![(0, "Line 1"), (9, "Line 2"), (18, "Line 3"),]);
}

#[test]
fn test_starting_with_newlines() {
  let text = "\n\r\nLine 1\nLine 2";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![(3, "Line 1"), (10, "Line 2"),]);
}

#[test]
fn test_ending_with_newlines() {
  let text = "Line 1\nLine 2\n\r\n";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![(0, "Line 1"), (7, "Line 2"),]);
}

#[test]
fn test_chinese_characters() {
  let text = "你好\n世界\r\n测试";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![(0, "你好"), (7, "世界"), (15, "测试"),]);
}

#[test]
fn test_only_newlines() {
  let text = "\n\r\n\r\n";
  let lines: Vec<_> = PosLines::new(text).collect();
  assert_eq!(lines, vec![]);
}

#[test]
fn test() {
  let text = "前面的文字\n```\n这是代码块\n包含多行\n```\n后面的文字";
  for (pos, line) in PosLines::new(text) {
    println!("{}: >{}<", pos, line);
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