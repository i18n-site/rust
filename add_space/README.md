# add_space

```rust
use unicode_script::{Script, UnicodeScript};

pub fn state(c: char) -> State {
  if c.is_whitespace() {
    return State::Space;
  }
  if matches!(
    c.script(),
    Script::Han
      | Script::Hiragana
      | Script::Katakana
      | Script::Thai
      | Script::Lao
      | Script::Khmer
      | Script::Myanmar
      | Script::Tibetan
  ) || ('０'..='９').contains(&c)
  {
    return State::Char;
  }
  if r##"'=!"#%*+,-.:：?@^`·—‘’“”…、。「」『』！，？"##.contains(c)
    || (c.len_utf8() > 1 && unic_emoji_char::is_emoji(c))
  {
    return State::Punctuation;
  }

  State::Letter
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum State {
  Space,
  Char,
  Letter,
  Punctuation,
}

pub fn add_space(txt: impl AsRef<str>) -> String {
  let txt = txt.as_ref();
  let mut r = String::new();
  let mut iter = txt.chars();

  if let Some(c) = iter.next() {
    r.push(c);
    let mut is_escape = c == '\\';
    let mut pre = state(c);
    for c in iter {
      if is_escape {
        is_escape = false;
        r.push(c);
        continue;
      }
      let s = state(c);
      match s {
        State::Char => {
          if pre == State::Letter {
            r.push(' ');
          }
          r.push(c);
        }
        State::Letter => {
          is_escape = c == '\\';
          if !is_escape && pre == State::Char {
            r.push(' ');
          }
          r.push(c);
        }
        _ => r.push(c),
      }
      pre = s;
    }
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
