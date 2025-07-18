mod i18n;
mod style;
use i18n::i18n;
pub use style::{LANG_STYLE, Style};
use txt_li::TxtLi;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
  Normal,
  Comment,
  MulitlineComment,
  Str,
  MulitlineStr,
}

pub fn getc(txt_li: &mut TxtLi, lang: &str, code: &str) {
  if lang == "i18n" {
    return crate::i18n(txt_li, code);
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
                txt_li.push_no_tran(&code[pre..end]);
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
              txt_li.push_no_tran(&code[pre..end]);
              pre = end;
              state = State::MulitlineComment;
            }
            _ => {}
          },
          State::MulitlineComment => match style {
            Style::CEnd | Style::Break => {
              let end = stmt.start();
              txt_li.push_md(&code[pre..end]);
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
              txt_li.push_md(&code[pre..end]);
              pre = end;
              state = State::Normal;
            }
          }
        }
      }
      let code = &code[pre..];
      if state == State::Comment {
        txt_li.push_md(code);
      } else {
        txt_li.push_no_tran(code);
      }
    })
  } else {
    txt_li.push_no_tran(code);
  };
}
// match i {
//   Style::
// }
