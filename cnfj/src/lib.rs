#![cfg_attr(docsrs, feature(doc_cfg))]

use std::{borrow::Cow, sync::LazyLock};

#[cfg(feature = "f2j")]
use concat_array::concat_array;

use daachorse::{CharwiseDoubleArrayAhoCorasick, CharwiseDoubleArrayAhoCorasickBuilder, MatchKind};

pub(crate) mod f;
pub(crate) mod j;

#[cfg(feature = "f2j")]
pub(crate) mod f2j;

#[cfg(feature = "f2j")]
const F2J_F: &[&str] = &concat_array!(f::F, f2j::F);
#[cfg(feature = "f2j")]
const F2J_J: &[&str] = &concat_array!(j::J, f2j::J);

#[cfg(feature = "f2j")]
// 安全保证：静态定义的字典通过单元测试验证，构建不会失败
static F2J_AC: LazyLock<CharwiseDoubleArrayAhoCorasick<usize>> = LazyLock::new(|| {
  CharwiseDoubleArrayAhoCorasickBuilder::new()
    .match_kind(MatchKind::LeftmostLongest)
    .build(F2J_F)
    .unwrap()
});

#[cfg(feature = "j2f")]
// 安全保证：静态定义的字典通过单元测试验证，构建不会失败
static J2F_AC: LazyLock<CharwiseDoubleArrayAhoCorasick<usize>> = LazyLock::new(|| {
  CharwiseDoubleArrayAhoCorasickBuilder::new()
    .match_kind(MatchKind::LeftmostLongest)
    .build(j::J)
    .unwrap()
});

pub fn replace_with_dict<'a>(
  text: &'a str,
  pma: &CharwiseDoubleArrayAhoCorasick<usize>,
  dict: &[&'static str],
) -> Cow<'a, str> {
  let mut matches = pma.leftmost_find_iter(text);

  if let Some(m) = matches.next() {
    let mut result = String::with_capacity(text.len());
    // 安全保证：匹配项起始位置均在有效字符边界内
    result.push_str(unsafe { text.get_unchecked(0..m.start()) });
    // 安全保证：匹配的模式索引值在字典长度范围内
    let val = unsafe { *dict.get_unchecked(m.value()) };
    result.push_str(val);
    let mut last_end = m.end();

    for m in matches {
      // 安全保证：匹配项起始位置均在有效字符边界内
      result.push_str(unsafe { text.get_unchecked(last_end..m.start()) });
      // 安全保证：匹配的模式索引值在字典长度范围内
      let val = unsafe { *dict.get_unchecked(m.value()) };
      result.push_str(val);
      last_end = m.end();
    }

    // 安全保证：last_end 不超过 text.len() 且在有效字符边界内
    result.push_str(unsafe { text.get_unchecked(last_end..) });
    Cow::Owned(result)
  } else {
    Cow::Borrowed(text)
  }
}

#[cfg(feature = "f2j")]
pub fn f2j(text: &str) -> Cow<'_, str> {
  replace_with_dict(text, &F2J_AC, F2J_J)
}

#[cfg(feature = "j2f")]
pub fn j2f(text: &str) -> Cow<'_, str> {
  replace_with_dict(text, &J2F_AC, &f::F)
}
