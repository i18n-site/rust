# varurl

```rust
use mdli::md_parse;
use varurl::VarUrl;

fn create_varurl() -> VarUrl {
  VarUrl::new(["https://i18n.com/"]).unwrap()
}

const FROM_LANG: &str = "/zh/";
const TO_LANG: &str = "/en/";

pub fn from_to(_pos: usize) -> Option<(&'static str, &'static str)> {
  Some((FROM_LANG, TO_LANG))
}

#[test]
fn test() {
  let input = r#"# Main Title
## Section 1\nHere's a paragraph with an ![inline image](https://i18n.com/zh/inline.png) and a [link](https://i18n.com/test/zh/doc.md) mixed in
"#;
  let mut mdli = md_parse(input);
  let varurl = create_varurl();
  varurl.replace(&mut mdli, |_| Some((FROM_LANG, TO_LANG)));
  let result = mdli.join();
  assert_eq!(result, input.replace(FROM_LANG, TO_LANG));

  let input = r#"
```markdown
![inline image](https://i18n.com/zh/inline.png)
```
"#;
  let mut mdli = md_parse(input);
  let varurl = create_varurl();
  varurl.replace(&mut mdli, from_to);
  let result = mdli.join();
  assert_eq!(result, input);
}

fn test_replace(title: &str, prefixes: &[&str], input: &str, expected: &str) {
  let varurl = VarUrl::new(prefixes).unwrap();
  let mut mdli = md_parse(input);
  varurl.replace(&mut mdli, from_to);
  let result = mdli.join();

  if result != expected {
    assert_eq!(
      result, expected,
      "\n❌ {}\n预期 {:?}\n实际 {:?}\n",
      title, expected, result
    );
  }
}

#[test]
fn test_all_cases() {
  let test_cases: &[(&str, &[&str], &str, &str)] = &[
    // 1. 基础链接测试
    (
      "简单链接测试",
      &["https://i18n.com/"],
      "[文档](https://i18n.com/zh/doc.md)",
      "[文档](https://i18n.com/en/doc.md)",
    ),
    (
      "简单图片测试",
      &["https://i18n.com/"],
      "![图片](https://i18n.com/zh/test.png)",
      "![图片](https://i18n.com/en/test.png)",
    ),
    // 2. 多域名测试
    (
      "多域名替换测试",
      &[
        "https://i18n.com/",
        "https://docs.i18n.com/",
        "https://img.i18n.com/",
        "//cdn.i18n.com/",
        "/docs/",
      ],
      r#"[主站点](https://i18n.com/zh/main.md)
[文档](https://docs.i18n.com/zh/api.md)
![图片](https://img.i18n.com/zh/test.png)
<img src="//cdn.i18n.com/zh/icon.png">
[本地文档](/docs/zh/guide.md)"#,
      r#"[主站点](https://i18n.com/en/main.md)
[文档](https://docs.i18n.com/en/api.md)
![图片](https://img.i18n.com/en/test.png)
<img src="//cdn.i18n.com/en/icon.png">
[本地文档](/docs/en/guide.md)"#,
    ),
    // 3. 不匹配测试
    (
      "不匹配域名测试",
      &["https://i18n.com/"],
      r#"[其他站点](https://other.com/zh/doc.md)
[错误前缀](https://wrong.i18n.com/zh/doc.md)
[正确链接](https://i18n.com/zh/doc.md)
[无语言](https://i18n.com/doc.md)"#,
      r#"[其他站点](https://other.com/zh/doc.md)
[错误前缀](https://wrong.i18n.com/zh/doc.md)
[正确链接](https://i18n.com/en/doc.md)
[无语言](https://i18n.com/doc.md)"#,
    ),
    // 4. 边界情况测试
    (
      "URL边界情况测试",
      &["https://i18n.com/", "/docs/"],
      r#"[带空格](https://i18n.com/zh/has space.md)
[带括号](/docs/zh/has(paren).md)
[带引号](https://i18n.com/zh/has"quote".md)
[带井号](https://i18n.com/zh/has#hash.md)
[带问号](https://i18n.com/zh/has?query=1)
[未闭合](https://i18n.com/zh/unclosed.md"#,
      r#"[带空格](https://i18n.com/en/has space.md)
[带括号](/docs/en/has(paren).md)
[带引号](https://i18n.com/en/has"quote".md)
[带井号](https://i18n.com/en/has#hash.md)
[带问号](https://i18n.com/en/has?query=1)
[未闭合](https://i18n.com/zh/unclosed.md"#,
    ),
    // 5. HTML属性测试
    (
      "HTML属性测试",
      &["https://i18n.com/"],
      r#"<img src="https://i18n.com/zh/img.png">
<a href="https://i18n.com/zh/link.md">
<img src="https://i18n.com/zh/img.png" />
<a href="https://i18n.com/zh/link.md"/>"#,
      r#"<img src="https://i18n.com/en/img.png">
<a href="https://i18n.com/en/link.md">
<img src="https://i18n.com/en/img.png" />
<a href="https://i18n.com/en/link.md"/>"#,
    ),
    // 6. 混合内容测试
    (
      "混合内容测试",
      &["https://i18n.com/", "/docs/"],
      r#"# 标题
[链接](https://i18n.com/zh/doc.md)和![图片](/docs/zh/img.png)
`代码中的[链接](https://i18n.com/zh/code.md)`
> 引用中的[链接](https://i18n.com/zh/quote.md)"#,
      r#"# 标题
[链接](https://i18n.com/en/doc.md)和![图片](/docs/en/img.png)
`代码中的[链接](https://i18n.com/zh/code.md)`
> 引用中的[链接](https://i18n.com/en/quote.md)"#,
    ),
    // 7. 代码块测试
    (
      "代码块测试",
      &["https://i18n.com/"],
      "```markdown
[代码块中的链接](https://i18n.com/zh/code.md)
```
[普通链接](https://i18n.com/zh/normal.md)
    [xx](https://i18n.com/zh/indent.md)
",
      "```markdown
[代码块中的链接](https://i18n.com/zh/code.md)
```
[普通链接](https://i18n.com/en/normal.md)
    [xx](https://i18n.com/en/indent.md)
",
    ),
    // 8. 复杂文档结构测试
    (
      "复杂文档结构测试",
      &["https://i18n.com/", "/docs/", "//cdn.i18n.com/"],
      "---
title: 测试文档
---

# 目录
* [章节1](/docs/zh/ch1.md)
  * [子章节](https://i18n.com/zh/sub.md)
* ![图片](//cdn.i18n.com/zh/img.png)

```js
// 代码中的链接不替换
const url = 'https://i18n.com/zh/code.md';
```

| 表格 | 链接 |
|------|------|
| 文档 | [链接1](https://i18n.com/zh/t1.md) |
| 图片 | ![图2](/docs/zh/t2.png) |

<div class=\"note\">
  <img src=\"https://i18n.com/zh/note.png\">
  <a href=\"/docs/zh/note.md\">注释</a>
</div>

1. [有序列表](https://i18n.com/zh/list.md)
2. [未闭合链接](https://i18n.com/zh/unclosed.md

> 引用
> ![图片](https://i18n.com/zh/quote.png)
>> [嵌套引用](/docs/zh/nested.md)",
      "---
title: 测试文档
---

# 目录
* [章节1](/docs/en/ch1.md)
  * [子章节](https://i18n.com/en/sub.md)
* ![图片](//cdn.i18n.com/en/img.png)

```js
// 代码中的链接不替换
const url = 'https://i18n.com/zh/code.md';
```

| 表格 | 链接 |
|------|------|
| 文档 | [链接1](https://i18n.com/en/t1.md) |
| 图片 | ![图2](/docs/en/t2.png) |

<div class=\"note\">
  <img src=\"https://i18n.com/en/note.png\">
  <a href=\"/docs/en/note.md\">注释</a>
</div>

1. [有序列表](https://i18n.com/en/list.md)
2. [未闭合链接](https://i18n.com/zh/unclosed.md

> 引用
> ![图片](https://i18n.com/en/quote.png)
>> [嵌套引用](/docs/en/nested.md)",
    ),
  ];
  for (title, prefixes, input, expected) in test_cases.iter() {
    test_replace(title, prefixes, input, expected);
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