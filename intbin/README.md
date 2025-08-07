# intbin

```rust
use std::borrow::Borrow;

use num_traits::ops::bytes::ToBytes;

pub fn bin_u64(bin: impl AsRef<[u8]>) -> u64 {
  let bin = bin.as_ref();
  let mut b = [0u8; 8];
  b[..bin.len()].copy_from_slice(bin);
  u64::from_le_bytes(b)
}

pub fn bin_u16(bin: impl AsRef<[u8]>) -> u16 {
  let bin = bin.as_ref();
  let mut b = [0u8; 2];
  b[..bin.len()].copy_from_slice(bin);
  u16::from_le_bytes(b)
}

pub fn to_bin(n: impl ToBytes) -> Box<[u8]> {
  let n = n.to_le_bytes();
  let n = n.borrow();
  let mut i = n.len();
  while i > 0 {
    let p = i - 1;
    if n[p] != 0 {
      break;
    }
    i = p;
  }
  Box::from(&n[..i])
}

pub fn u8_bin(n: u8) -> Box<[u8]> {
  if n == 0 {
    return [].into();
  }
  [n].into()
}

pub fn bin_u8(bin: impl AsRef<[u8]>) -> u8 {
  let bin = bin.as_ref();
  if bin.is_empty() {
    return 0;
  };
  bin[0]
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
