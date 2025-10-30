#![cfg_attr(docsrs, feature(doc_cfg))]

use daachorse::CharwiseDoubleArrayAhoCorasick;

pub mod f;
pub mod j;

#[cfg(feature = "f2j")]
pub mod f2j;

#[cfg(feature = "f2j")]
use concat_array::concat_array;

#[cfg(feature = "f2j")]
const F2J_F: &[&str] = &concat_array!(f::F, f2j::F);
#[cfg(feature = "f2j")]
const F2J_J: &[&str] = &concat_array!(j::J, f2j::J);

#[cfg(feature = "f2j")]
#[static_init::dynamic]
static F2J_AC: CharwiseDoubleArrayAhoCorasick<usize> =
  daachorse::CharwiseDoubleArrayAhoCorasickBuilder::new()
    .match_kind(daachorse::MatchKind::LeftmostLongest)
    .build(F2J_F)
    .unwrap();

#[cfg(feature = "j2f")]
#[static_init::dynamic]
static J2F_AC: CharwiseDoubleArrayAhoCorasick<usize> =
  daachorse::CharwiseDoubleArrayAhoCorasickBuilder::new()
    .match_kind(daachorse::MatchKind::LeftmostLongest)
    .build(j::J)
    .unwrap();

pub fn replace_with_dict(
  text: &str,
  pma: &CharwiseDoubleArrayAhoCorasick<usize>,
  dict: &[&'static str],
) -> String {
  let mut result = String::new();
  let mut last_end = 0;
  for m in pma.leftmost_find_iter(text) {
    result.push_str(&text[last_end..m.start()]);
    result.push_str(dict[m.value()]);
    last_end = m.end();
  }
  result.push_str(&text[last_end..]);
  result
}

#[cfg(feature = "f2j")]
pub fn f2j(text: impl AsRef<str>) -> String {
  replace_with_dict(text.as_ref(), &F2J_AC, F2J_J)
}

#[cfg(feature = "j2f")]
pub fn j2f(text: impl AsRef<str>) -> String {
  replace_with_dict(text.as_ref(), &J2F_AC, &f::F)
}
