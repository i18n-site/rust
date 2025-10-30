#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "yml")]
pub mod yml;

use std::{
  borrow::Borrow,
  collections::{HashMap, HashSet},
};

use daachorse::{CharwiseDoubleArrayAhoCorasick, CharwiseDoubleArrayAhoCorasickBuilder, MatchKind};
use htmlize::{escape_attribute, unescape_attribute};
use tag_replace::{TagReplace, word_push};
use thiserror::Error;
use unicode_categories::UnicodeCategories;
use unicode_segmentation::UnicodeSegmentation;

#[static_init::dynamic]
pub static CODE: TagReplace = TagReplace::new("code", "v");

#[derive(Error, Debug)]
pub enum Error {
  #[error("daachorse: {0}")]
  Daachorse(daachorse::errors::DaachorseError),
}

fn capitalize_first_letter(s: impl AsRef<str>) -> String {
  let mut chars = s.as_ref().chars();
  match chars.next() {
    Some(first_char) => first_char.to_uppercase().to_string() + chars.as_str(),
    None => String::new(),
  }
}

pub struct Term {
  pub map: HashMap<String, String>,
  pub ac: CharwiseDoubleArrayAhoCorasick<usize>,
}

impl Term {
  pub fn load<'a, S1: AsRef<str> + 'a, S2: AsRef<str> + 'a, T: Borrow<(S1, S2)>>(
    from_to_iter: impl IntoIterator<Item = T>,
  ) -> Result<Self, Error> {
    let mut map = HashMap::new();

    let mut exist = HashSet::new();

    for i in from_to_iter {
      let (key, value) = i.borrow();
      let lower_key = key.as_ref().to_lowercase();
      if lower_key.is_empty() || exist.contains(&lower_key) {
        continue;
      }
      exist.insert(lower_key.clone());

      let value = value.as_ref().into();
      map.insert(lower_key, value);
    }

    match CharwiseDoubleArrayAhoCorasickBuilder::new()
      .match_kind(MatchKind::LeftmostLongest)
      .build(map.keys())
    {
      Ok(ac) => Ok(Term { map, ac }),
      Err(err) => Err(Error::Daachorse(err)),
    }
  }

  pub fn restore(&self, txt: impl AsRef<str>) -> String {
    let txt = txt.as_ref();
    CODE.replace(txt, |li, _origin, val: &str| {
      word_push(li, unescape_attribute(val));
    })
  }

  pub fn replace(&mut self, txt: impl AsRef<str>) -> Option<String> {
    let txt = txt.as_ref();
    let txt_lower = txt.to_lowercase();
    let mut li = vec![];
    let mut pos = 0;
    for mat in self.ac.leftmost_find_iter(&txt_lower) {
      let start = mat.start();
      let end = mat.end();
      let matched = &txt_lower[start..end];

      macro_rules! is_word {
        ($prev:expr, $next:expr) => {
          if let Some(last) = $prev.chars().last()
            && let Some(first) = $next.chars().next()
          {
            let t = last.to_string() + &first.to_string();
            if !t.contains('_') {
              let mut t = t.split_word_bounds();
              match (t.next(), t.next()) {
                (Some(_), None) => continue,
                _ => {}
              };
            }
          }
        };
      }

      if let Some(val) = self.map.get(matched) {
        let mut val = val.to_owned();

        // 如果的一个单词的内部, 不替换
        is_word!(&txt[..start], matched);
        is_word!(matched, &txt[end..]);

        if start > pos {
          li.push(txt[pos..start].to_owned());
        }

        let org = &txt[start..end];
        if let Some(c) = org.chars().next()
          && (
            c.is_uppercase() || start == 0
            // 行首大写
          )
        {
          let pos = c.to_string().len();
          if org.len() > 1 && org[pos..].chars().all(char::is_uppercase) {
            val = val.to_uppercase();
          } else {
            val = capitalize_first_letter(val);
          }
        } else if let Some(last) = txt[..start].trim_end().chars().last()
          && !"{}()_*[]~".contains(last)
          && last.is_punctuation()
        {
          val = capitalize_first_letter(val)
        }

        let val = escape_attribute(val);
        li.push(format!(r#"<code v="{val}">{matched}</code>"#));
      }
      pos = end;
    }
    if !li.is_empty() {
      if pos < txt.len() {
        li.push(txt[pos..].into());
      }
      let r = li.concat();
      return Some(r);
    }
    None
  }
}

impl PartialEq for Term {
  fn eq(&self, other: &Self) -> bool {
    self.map == other.map
  }
}
