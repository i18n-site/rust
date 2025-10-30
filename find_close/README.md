# find_close

```rust
/*
输入html和tag,返回此tag的闭合标签位置。
注意: tag可能有嵌套，比如<code>abc<code>test</code>123</code>, 需要找到对应的闭合返回。
参数 htm_li 是迭代器，usize代表了偏移量，&str是行内容。
返回值为行的偏移量+闭合标记结尾的位置。
如果找不到闭合位置,返回0。
用状态机实现，标签可能不规范，比如 </ pre >
*/

#[derive(Debug)]
pub struct FindClose<'a> {
  pub stack: usize,
  pub tag: &'a str,
}

impl<'a> FindClose<'a> {
  pub fn new(tag: &'a str) -> Self {
    Self { stack: 0, tag }
  }

  pub fn find(&mut self, htm: impl AsRef<str>) -> Option<usize> {
    let htm = htm.as_ref();
    let tag = self.tag;

    if tag == "br" {
      return htm.find(">").map(|i| i + 1);
    }

    let htm = htm.to_lowercase();
    let mut iter = htm.char_indices();
    while let Some((_, c)) = iter.next() {
      if c == '<' {
        while let Some((_, c)) = iter.next() {
          if c.is_whitespace() {
            continue;
          }
          if c == '/' {
            let mut t = String::new();
            for (pos, c) in iter.by_ref() {
              if c == '>' {
                if t.trim() == tag {
                  if self.stack == 0 {
                    return Some(pos + 1);
                  }
                  self.stack -= 1;
                }
                break;
              }
              t.push(c);
            }
          } else {
            let mut t = String::from(c);
            for (_, c) in iter.by_ref() {
              if c == '>' {
                if t.trim() == tag {
                  self.stack += 1;
                }
                break;
              }
              t.push(c);
            }
          }
          break;
        }
      }
    }

    // 没有找到匹配的闭合标签
    None
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
