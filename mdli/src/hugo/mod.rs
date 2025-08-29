// https://gohugo.io/content-management/front-matter/#front-matter-formats

use crate::{MdLi, kind::Kind};

#[derive(Default)]
pub struct MdHugo(MdLi);

impl MdHugo {
  pub fn push(&mut self, txt: &str) {
    self.0.push(Kind::HugoHead, txt);
  }
  pub fn push_txt(&mut self, txt: &str) {
    super::md_parse(txt, &mut self.0);
    // self.0.push_txt(Kind::Txt, txt);
  }
}

pub const TRAN: [&str; 4] = ["description", "title", "summary", "brief"];

#[macro_export]
macro_rules! hugo_head {
  ($($fn:ident),*) => {
    $(
mod $fn;
pub use $fn::$fn;
    )*

pub fn remove_head(
  md: &str,
) -> (&str, MdLi) {
  let mut mdli = MdHugo::default();
$(
  if let Some(md) = $fn(md, &mut mdli) {
    return (md,mdli.0);
  }
)*
  (md, mdli.0)
}
  };
}

hugo_head!(yaml, toml);

#[derive(PartialEq)]
pub enum State {
  Check,
  Ignore,
}

pub fn prefix(md: &str, flag: char) -> Option<usize> {
  let mut iter = md.char_indices();
  for (_, i) in iter.by_ref() {
    if i.is_whitespace() {
      continue;
    }
    if i != flag {
      return None;
    }
    break;
  }
  let mut n = 1;
  for (p, i) in iter {
    if i == flag {
      n += 1;
    } else {
      if n >= 3 && ('\r' == i || '\n' == i) {
        return Some(p);
      }
      break;
    }
  }

  None
}

pub fn whitespace(
  md: &str,
) -> (
  usize,
  bool, // has_break
) {
  let mut has_break = false;
  for (p, i) in md.char_indices() {
    if i.is_whitespace() {
      if i == '\r' || i == '\n' {
        has_break = true;
      }
    } else {
      return (p, has_break);
    }
  }
  // file end
  (md.len(), true)
}

pub fn extract<'a>(
  flag: char,
  mut md: &'a str,
  mdli: &mut MdHugo,
  func: impl FnOnce(&'a str, &mut MdHugo),
) -> Option<
  &'a str, // md
> {
  if let Some(pos) = prefix(md, flag) {
    macro_rules! push {
      ($pos:expr) => {{
        let pos = $pos;
        if pos > 0 {
          mdli.push(&md[..pos]);
          md = &md[pos..];
        }
      }};
    }
    push!(pos);
    let (pos, has_break) = whitespace(md);
    push!(pos);
    if has_break {
      let mut state = State::Check;

      for (pos, i) in crate::pos::chars(md) {
        if i == '\r' || i == '\n' {
          state = State::Check;
        }
        if state == State::Check
          && let Some(len) = prefix(&md[pos..], flag)
        {
          let end = pos + len;
          let (wlen, has_break) = whitespace(&md[end..]);
          if !has_break {
            continue;
          }
          let end = end + wlen;

          if pos > 0 {
            let pos = crate::pos::trim_end(&md[..pos], char::is_whitespace);
            func(&md[..pos], mdli);
            mdli.push(&md[pos..end]);
          }
          return Some(&md[end..]);
        }
        state = State::Ignore;
      }
    }
    Some(md)
  } else {
    None
  }
}
