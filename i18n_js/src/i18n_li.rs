#[derive(Debug)]
pub struct I18nLi(pub Vec<String>);

impl I18nLi {
  pub fn replace(&mut self, txt: impl AsRef<str>) -> (String, usize) {
    let extract = rvar::extract(txt.as_ref());
    (
      extract.replace(&txt, |key| {
        format!(
          "${{{}}}",
          if let Some(key) = key.strip_prefix("I18N.") {
            format!("I[{}]", self.pos(key))
          } else {
            key.into()
          }
        )
      }),
      extract.range.len(),
    )
  }

  pub fn pos(&mut self, key: impl AsRef<str>) -> usize {
    let key = key.as_ref();
    match self.0.iter().position(|i| i == key) {
      Some(i) => i,
      None => {
        let i = self.0.len();
        self.0.push(key.into());
        i
      }
    }
  }
}
