use aok::{OK, Void};
use located_yaml::{
  YamlElt::{Hash, String as Str},
  YamlLoader,
};
use txt_li::TxtLi;

use crate::{Parse, TRAN};

pub fn parse<P: Parse>(txt_li: &mut TxtLi, txt: String) -> Void {
  if let Ok(res) = YamlLoader::load_from_str(&txt) {
    let byte_li: Vec<_> = txt.char_indices().map(|i| i.0).collect();
    if byte_li.is_empty() {
      return OK;
    }
    let mut pre = 0;
    let mut is_tran_tag = false;

    for doc in res.docs {
      if let Hash(yml) = doc.yaml {
        for (k, v) in yml {
          if is_tran_tag {
            let end = byte_li[k.marker.index];

            let mut t = &txt[pre..end];
            let mut trim_end = t.trim_end();

            if t.starts_with("'") || t.starts_with("\"") {
              let c = &t[..1];
              if trim_end.ends_with(c) {
                txt_li.push_no_tran(c);
                trim_end = &trim_end[1..trim_end.len() - 1];
                t = &t[1..];
              }
            };

            if !trim_end.is_empty() {
              P::parse(txt_li, trim_end.lines())?;
              {
                let end_str = t[trim_end.len()..].to_owned();
                if !end_str.is_empty() {
                  txt_li.push_no_tran(end_str);
                }
              }
              pre = end;
            }
            is_tran_tag = false;
          } else if let Str(k) = k.yaml
            && let Str(_) = v.yaml
            && TRAN.contains(&k.as_str())
          {
            let end = byte_li[v.marker.index];
            txt_li.push_no_tran(&txt[pre..end]);
            pre = end;
            is_tran_tag = true;
          }
        }
      }
    }
    let remain = &txt[pre..];
    if is_tran_tag {
      P::parse(txt_li, remain.lines())?;
    } else {
      txt_li.push_no_tran(remain.to_owned());
    }
    txt_li.push_no_tran("\n");
  }
  OK
}
