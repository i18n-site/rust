mod i18n;
mod style;
use i18n::i18n;
pub use style::{Style, LANG_STYLE};
use tp::TxtPos;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
  Normal,
  Comment,
  MulitlineComment,
  Str,
  MulitlineStr,
}

pub fn getc<'a>(lang: &str, code: &'a str, txtpos: &mut TxtPos<'a>) {
  if lang == "i18n" {
    return crate::i18n(code, txtpos);
  }
  if let Some(style) = LANG_STYLE.get(lang) {
    style.with(|style| {
      let mut pre = 0;
      let mut state = State::Normal;
      let mut flag = "";
      for stmt in style.leftmost_find_iter(code) {
        let style = stmt.value();

        macro_rules! is_escape {
          () => {
            let start = stmt.start();
            if Some('\\') == code[..start].chars().rev().next() {
              continue;
            } else {
              state = State::Normal;
            }
          };
        }

        match state {
          State::Normal => match style {
            Style::C => {
              let pos = stmt.start();
              if !(code[..pos].trim().is_empty() && code.starts_with("#!")) {
                let end = stmt.end();
                txtpos.push(&code[pre..end]);
                pre = end;
                state = State::Comment;
              }
            }
            Style::Str => {
              let start = stmt.start();
              let end = stmt.end();
              flag = &code[start..end];
              state = State::Str;
            }
            Style::StrBegin => {
              state = State::MulitlineStr;
            }
            Style::CBegin => {
              let end = stmt.end();
              txtpos.push(&code[pre..end]);
              pre = end;
              state = State::MulitlineComment;
            }
            _ => {}
          },
          State::MulitlineComment => match style {
            Style::CEnd | Style::Break => {
              let end = stmt.start();
              txtpos.push_txt_line(&code[pre..end]);
              pre = end;
              if style == Style::CEnd {
                state = State::Normal;
              }
            }
            _ => {}
          },
          State::MulitlineStr => {
            if Style::StrEnd == style {
              is_escape!();
            }
          }
          State::Str => {
            if Style::Str == style {
              let now_flag = &code[stmt.start()..stmt.end()];
              if now_flag == flag {
                is_escape!();
              }
            }
          }
          State::Comment => {
            if Style::Break == style {
              let end = stmt.start();
              txtpos.push_txt_line(&code[pre..end]);
              pre = end;
              state = State::Normal;
            }
          }
        }
      }
      let code = &code[pre..];
      if state == State::Comment {
        txtpos.push_txt_line(code);
      } else {
        txtpos.push(code);
      }
    })
  } else {
    txtpos.push(code);
  };
}
// match i {
//   Style::
// }
