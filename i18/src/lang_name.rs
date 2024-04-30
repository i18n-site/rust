use std::collections::HashMap;

use lang::Lang;

pub fn lang_name_li(id_li: impl AsRef<[u16]>, lang_str: &HashMap<Lang, String>) -> Vec<String> {
  let id_li = id_li.as_ref();
  let mut r = Vec::with_capacity(id_li.len());

  for i in id_li.iter() {
    r.push(if let Ok::<Lang, _>(lang) = (*i).try_into() {
      if let Some(name) = lang_str.get(&lang) {
        name.clone()
      } else {
        lang.code().into()
      }
    } else {
      panic!("miss lang id: {} , please upgrade", i)
    })
  }
  r
}
