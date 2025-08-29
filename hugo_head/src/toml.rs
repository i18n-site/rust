use aok::{OK, Void};
use taplo::{
  rowan::{NodeOrToken, SyntaxKind},
  syntax::SyntaxKind as S,
};
use txt_li::TxtLi;

use crate::{Parse, TRAN};

macro_rules! syntax_kind {
  ($($name:ident),*) => {
    $(
    pub const $name: SyntaxKind = SyntaxKind(S::$name as _);
    )*
  };
}

syntax_kind!(
  KEY,
  VALUE,
  ENTRY,
  STRING,
  MULTI_LINE_STRING,
  STRING_LITERAL,
  MULTI_LINE_STRING_LITERAL
);

pub fn parse<P: Parse>(txt_li: &mut TxtLi, txt: String) -> Void {
  let doc = taplo::parser::parse(&txt);

  let mut pos = 0;
  let mut pre = 0;
  for i in doc.green_node.children() {
    if let NodeOrToken::Node(i) = i
      && ENTRY == i.kind()
    {
      let mut iter = i.children();
      let mut need_tran = false;

      for j in iter.by_ref() {
        pos += usize::from(j.text_len());
        if let KEY = j.kind() {
          let t = j.to_string();
          if TRAN.contains(&t.trim()) {
            need_tran = true;
            break;
          }
        }
      }
      if need_tran {
        for j in iter.by_ref() {
          if let NodeOrToken::Node(s) = j
            && VALUE == j.kind()
          {
            for k in s.children() {
              let mut push = |len: usize| {
                let klen = usize::from(k.text_len());
                let len2 = len * 2;
                if klen > len2 {
                  txt_li.push_no_tran(&txt[pre..pos + len]);

                  let k = k.to_string();
                  let end = k.len() - len;

                  let content = &k[len..end];

                  let trim_start = content.trim_start();
                  let prefix = &content[..content.len() - trim_start.len()];

                  if !prefix.is_empty() {
                    txt_li.push_no_tran(prefix);
                  }

                  let trim_end = trim_start.trim_end();
                  if !trim_end.is_empty() {
                    P::parse(txt_li, trim_end.lines())?;
                    let reamin = &trim_start[trim_end.len()..];
                    if !reamin.is_empty() {
                      txt_li.push_no_tran(reamin);
                    }
                    pre = pos + end;
                  }
                }
                pos += klen;
                OK
              };
              match k.kind() {
                STRING | STRING_LITERAL => {
                  push(1)?;
                }
                MULTI_LINE_STRING | MULTI_LINE_STRING_LITERAL => {
                  push(3)?;
                }
                _ => {
                  pos += usize::from(k.text_len());
                }
              }
            }
          } else {
            pos += usize::from(j.text_len());
          }
        }
      }
    } else {
      pos += usize::from(i.text_len());
    }
  }

  let remain = &txt[pre..];

  if !remain.is_empty() {
    txt_li.push_no_tran(remain);
  }
  txt_li.push_no_tran("\n");
  OK
}
