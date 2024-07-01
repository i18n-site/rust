use lang::LANG_CODE;

pub fn lang_name_li(id_li: impl AsRef<[u16]>) -> Vec<&'static str> {
  let id_li = id_li.as_ref();
  let mut r = Vec::with_capacity(id_li.len());

  for i in id_li {
    let i = *i as usize;
    if i < LANG_CODE.len() {
      r.push(LANG_CODE[i])
    } else {
      panic!("miss lang id: {} , please upgrade", i)
    }
  }
  r
}
