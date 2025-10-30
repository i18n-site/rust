# txt_chunk

```rust
pub type ChunkLi = Vec<Vec<String>>;

pub fn txt_chunk<S: Into<String>>(li: impl IntoIterator<Item = S>, limit: usize) -> ChunkLi {
  let mut r = vec![];
  let mut t = vec![];
  let mut len = 0;
  for i in li {
    let i = i.into();
    let i_len = 1 + i.len();
    let next_len = len + i_len;
    if next_len > limit {
      r.push(t);
      if i_len < limit {
        t = vec![i];
        len = i_len;
      } else {
        let mut end = limit;
        while !i.is_char_boundary(end) && end > 0 {
          end -= 1;
        }
        if end > 0 {
          r.push(vec![i[..end].into()]);
        }
        t = vec![];
        len = 0;
      }
    } else {
      len = next_len;
      t.push(i);
    }
  }

  if !t.is_empty() {
    r.push(t);
  }

  r
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
