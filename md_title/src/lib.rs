#![feature(const_trait_impl)]

use std::io::Cursor;

pub fn title_trim(input: &str) -> &str {
  let input = input.trim();
  if let Some(t) = input.strip_prefix("<h1") {
    if let Some(i) = t.chars().next() {
      if i.is_whitespace() || i == '>' {
        return input;
      }
    }
  }
  remove_h(input).unwrap_or("")
}

pub fn remove_h(input: &str) -> Option<&str> {
  if let Some(t) = input.strip_prefix("#") {
    for (pos, i) in t.char_indices() {
      if !i.is_whitespace() && i != '#' {
        return Some(t[pos..].trim_start());
      }
    }
  }
  None
}

pub const EMPTY: String = String::new();

pub fn md_title_txt(md: &str) -> String {
  for i in md.lines() {
    let t = title_trim(i);
    if !t.is_empty() {
      let t = Cursor::new(t);
      let t = html2text::from_read(t, usize::MAX);
      let t = t.trim();
      let t = remove_h(t).unwrap_or(t);
      if !t.is_empty() {
        return t.into();
      }
    }
  }
  EMPTY
}

pub fn md_title(md: &str) -> String {
  htmlize::escape_text(md_title_txt(md)).into()
}
