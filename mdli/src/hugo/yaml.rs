use located_yaml::{
  YamlElt::{Hash, String as Str},
  YamlLoader,
};

use crate::hugo::{extract, TRAN};

pub fn push<'a>(md: &'a str, mdli: &mut super::MdHugo, pre: usize, begin: usize, end: usize) {
  let pos = begin + crate::pos::trim_start(&md[begin..end], crate::whitespace_or_quote);

  if pos != pre {
    mdli.push(&md[pre..pos]);
  }

  let epos = pos + crate::pos::trim_end(&md[pos..end], crate::whitespace_or_quote);

  if pos != epos {
    mdli.push_txt(&md[pos..epos]);
  }

  if end != epos {
    mdli.push(&md[epos..end]);
  }
}

pub fn yaml<'a>(md: &'a str, mdli: &mut super::MdHugo) -> Option<&'a str> {
  extract('-', md, mdli, |mut txt, mdli| {
    if let Ok(res) = YamlLoader::load_from_str(txt) {
      let byte_li: Vec<_> = txt.char_indices().map(|i| i.0).collect();

      let mut pre = 0;
      let mut pre_is_tran_tag = None;
      for doc in res.docs {
        if let Hash(yml) = doc.yaml {
          for (k, v) in yml {
            if let Some(tag) = pre_is_tran_tag {
              let begin = byte_li[tag];
              let end = byte_li[k.marker.index];

              push(txt, mdli, pre, begin, end);
              pre = end;
              pre_is_tran_tag = None;
            }

            if let Str(k) = k.yaml {
              if let Str(_) = v.yaml {
                if TRAN.contains(&k.as_str()) {
                  pre_is_tran_tag = Some(v.marker.index);
                }
              }
            }
          }
        }
      }
      if let Some(tag) = pre_is_tran_tag {
        let begin = byte_li[tag];
        let end = txt.len();
        push(txt, mdli, pre, begin, end);
        return;
      }
      txt = &txt[pre..];
    }
    mdli.push(txt);
  })
}
