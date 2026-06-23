# intbin

```rust
use std::borrow::Borrow;

use num_traits::ops::bytes::ToBytes;

/// 内部泛型辅助函数：将字节切片安全且无越界检查地转换为固定长度数组
#[inline]
fn bin_to_int<const N: usize>(bin: &[u8]) -> [u8; N] {
  let mut b = [0u8; N];
  let len = bin.len().min(N);
  // SAFETY: `len` 不会超过 `N`（由 `.min(N)` 保证），也不会超过 `bin.len()`，
  // 因此 `..len` 范围对 `b` 和 `bin` 均 100% 安全。
  unsafe {
    b.get_unchecked_mut(..len)
      .copy_from_slice(bin.get_unchecked(..len));
  }
  b
}

pub fn bin_u64(bin: impl AsRef<[u8]>) -> u64 {
  u64::from_le_bytes(bin_to_int(bin.as_ref()))
}

pub fn bin_u16(bin: impl AsRef<[u8]>) -> u16 {
  u16::from_le_bytes(bin_to_int(bin.as_ref()))
}

pub fn to_bin(n: impl ToBytes) -> Box<[u8]> {
  let n = n.to_le_bytes();
  let bytes = n.borrow();
  let len = bytes.iter().rposition(|&x| x != 0).map_or(0, |i| i + 1);
  // SAFETY: `len` 不可能超过 `bytes.len()`，因此 `..len` 是安全的
  let slice = unsafe { bytes.get_unchecked(..len) };
  Box::from(slice)
}

pub fn u8_bin(n: u8) -> Box<[u8]> {
  if n == 0 {
    Box::default()
  } else {
    Box::from([n])
  }
}

pub fn bin_u8(bin: impl AsRef<[u8]>) -> u8 {
  bin.as_ref().first().copied().unwrap_or(0)
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

* [i18 : MarkDown 命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖 （ 如果这段原文没有被修改 ）。

* [i18n.site : MarkDown 多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。
