# tag_replace

```rust
use unicode_segmentation::UnicodeSegmentation;

// 判断是否需要插入空格
pub fn word_push(pre: &mut String, txt: impl Into<String>) {
  let txt = txt.into();
  if let Some(last) = pre.chars().last()
    && let Some(first) = txt.chars().next()
  {
    let t = last.to_string() + &first.to_string();
    if !t.contains('_') {
      let mut t = t.split_word_bounds();
      if let (Some(_), None) = (t.next(), t.next()) {
        pre.push(' ')
      };
    }
  }
  pre.push_str(&txt);
}

pub struct TagReplace {
  pub start_tag: String,
  pub end_tag: String,
}

impl TagReplace {
  pub fn new(tag: impl AsRef<str>, attr: impl AsRef<str>) -> Self {
    let tag = tag.as_ref();
    let attr = attr.as_ref();
    Self {
      start_tag: format!("<{tag} {attr}=\""),
      end_tag: format!("</{tag}>"),
    }
  }

  /// replacer(前文, 原文, code_id)
  pub fn replace(
    &self,
    txt: impl AsRef<str>,
    replacer: impl Fn(&mut String, &str, &str),
  ) -> String {
    let mut result = String::new();
    let mut txt = txt.as_ref();

    let start_tag_len = self.start_tag.len();
    while let Some(start_tag) = txt.find(&self.start_tag) {
      word_push(&mut result, &txt[..start_tag]);

      let txt2 = &txt[start_tag..];
      if let Some(pos) = txt2[start_tag_len..].find("\">") {
        let end = start_tag_len + pos;
        let val = &txt2[start_tag_len..end];
        let begin = end + 2;
        if let Some(pos) = txt2[begin..].find(&self.end_tag) {
          let offset = begin + pos + self.end_tag.len();
          let org = &txt[start_tag..offset + start_tag];
          replacer(&mut result, org, val);
          txt = &txt2[offset..];
          continue;
        }
      }

      result.push_str(&txt[..start_tag_len]);
      txt = &txt[start_tag_len..];
    }

    result.push_str(txt);
    result
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
