use lang::Lang;

pub async fn tran(from_lang: Lang, to_lang_li: Vec<Lang>, txt: &str) -> Vec<String> {
  let mut r = Vec::with_capacity(to_lang_li.len());
  dbg!((&from_lang, &txt));
  for _ in to_lang_li {
    r.push("".into());
    // r.push(tran(from_lang, i, txt).await)
  }
  r
}
