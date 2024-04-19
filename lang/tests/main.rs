#[test]
fn test() -> aok::Result<()> {
  use lang::Lang;

  for i in ["zh", "zh-CN", "zh-TW", "zh-tw"] {
    let lang: Lang = i.try_into()?;
    dbg!((i, lang, lang.code()));
  }
  aok::OK
}

/*
#[cfg(feature = "macro")]
mod test_macro {
}
*/
