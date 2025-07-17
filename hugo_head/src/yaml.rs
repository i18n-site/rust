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

            let t = &txt[pre..end];
            let trim_end = t.trim_end();

            P::parse(txt_li, trim_end.lines())?;

            {
              let mut end_str = t[trim_end.len()..].to_owned();
              if !end_str.is_empty() {
                // md parse 会多插入一个回车
                end_str.pop();
                if !end_str.is_empty() {
                  txt_li.push_no_tran(end_str);
                }
              }
            }

            pre = end;
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
      txt_li.push_tran(remain.to_owned() + "\n");
    }
  }
  OK
}
