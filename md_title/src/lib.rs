#![feature(const_trait_impl)]

use std::{
  io::{BufRead, BufReader, Cursor},
  path::Path,
};

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

pub fn line_title(line: &str) -> String {
  let t = title_trim(line);
  if !t.is_empty() {
    let t = Cursor::new(t);
    if let Ok(t) = xerr::ok!(html2text::from_read(t, usize::MAX)) {
      let t = t.trim();
      let t = remove_h(t).unwrap_or(t);
      if !t.is_empty() {
        return t.into();
      }
    }
  }
  EMPTY
}

pub fn md_title_txt(md: &str) -> String {
  for i in md.lines() {
    let title = line_title(i);
    if !title.is_empty() {
      return title;
    }
  }
  EMPTY
}

pub fn md_title(md: &str) -> String {
  htmlize::escape_text(md_title_txt(md)).into()
}

pub fn md_title_from_path(path: impl AsRef<Path>) -> std::io::Result<String> {
  let path = path.as_ref();
  let fp = BufReader::new(std::fs::File::open(path)?);
  for i in fp.lines().map_while(Result::ok) {
    let title = line_title(&i);
    if !title.is_empty() {
      return Ok(htmlize::escape_text(title).into());
    }
  }
  Ok(EMPTY)
}
