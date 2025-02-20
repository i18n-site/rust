use taplo::{
  parser::parse,
  rowan::{NodeOrToken, SyntaxKind},
  syntax::SyntaxKind as S,
};

use super::{extract, TRAN};

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

pub fn toml<'a>(md: &'a str, mdli: &mut super::MdHugo) -> Option<&'a str> {
  extract('+', md, mdli, |txt, mdli| {
    let doc = parse(txt);

    let mut pos: usize = 0;
    let mut pre: usize = 0;

    // let s: S = unsafe { std::mem::transmute(9u16) };
    // let s: S = unsafe { std::mem::transmute(10u16) };
    // let s: S = unsafe { std::mem::transmute(11u16) };

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
                macro_rules! push {
                  ($len:expr) => {{
                    let len = $len;
                    let klen = usize::from(k.text_len());
                    let len2 = len * 2;
                    if klen > len2 {
                      let mut txt_len = klen - len;
                      let k = &k.to_string()[len..txt_len];
                      txt_len -= len;
                      txt_len -= crate::pos::trim_start(k, char::is_whitespace);

                      if txt_len > 0 {
                        pos += len;
                        pos += (klen - len2 - txt_len);
                        mdli.push(&txt[pre..pos]);

                        let end = pos + txt_len;

                        mdli.push_txt(&txt[pos..end]);
                        pos = end + len;
                        mdli.push(&txt[end..pos]);
                        pre = pos;
                        continue;
                      }
                    }
                    pos += klen;
                  }};
                }
                match k.kind() {
                  STRING | STRING_LITERAL => {
                    push!(1);
                  }
                  MULTI_LINE_STRING | MULTI_LINE_STRING_LITERAL => {
                    push!(3);
                  }
                  _ => {
                    pos += usize::from(k.text_len());
                  }
                }
              }
              continue;
            }
            pos += usize::from(j.text_len());
          }
        } else {
          pos += iter.map(|j| usize::from(j.text_len())).sum::<usize>();
        }
        continue;
      }
      pos += usize::from(i.text_len());
    }

    if pre != txt.len() {
      mdli.push(&txt[pre..]);
    }
  })
}
