# doc_chunk

```rust
#[derive(Debug, Clone, Copy)]
pub struct DocChunk {
  pub limit: usize,
}

impl DocChunk {
  pub fn new(limit: usize) -> Self {
    Self { limit }
  }

  pub fn parse(&self, txt: impl AsRef<str>) -> Vec<String> {
    let limit = self.limit;

    let mut result = vec![];

    let mut count = 0;

    let mut tmp = String::new();

    macro_rules! parse {
      ($line:expr) => {{
        let line = $line;
        let len = line.len();

        let line = line.trim_end();
        if line.trim_start().is_empty() {
          continue;
        }
        let n = count + len + 1;

        let tmp_not_empty = !tmp.is_empty();
        if n > limit {
          if tmp_not_empty {
            result.push(tmp.to_owned());
          }
          if len > limit {
            let mut end = limit;
            while !line.is_char_boundary(end) {
              end -= 1;
            }
            result.push(line[..end].into());

            tmp = String::new();
          } else {
            tmp = line.into();
          }
          count = tmp.len();
        } else {
          count = n;
          if tmp_not_empty {
            tmp.push('\n');
          }
          tmp.push_str(line);
        }
      }};
    }

    for line in txt.as_ref().lines() {
      if line.len() > limit {
        use unicode_segmentation::UnicodeSegmentation;
        let mut t = String::new();
        for i in line.unicode_sentences() {
          let i = i.trim();
          let len = i.len();
          if len > 0 {
            t.push_str(i);
            if i.len() > 16 && t.len() > 95 {
              parse!(&t);
              t = String::new();
            }
          }
        }
        if !t.is_empty() {
          parse!(&t);
        }
      } else {
        parse!(line);
      }
    }

    if !tmp.is_empty() {
      result.push(tmp);
    }
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
