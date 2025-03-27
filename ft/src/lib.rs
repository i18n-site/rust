#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Debug, Hash)]
pub struct FromTo {
  pub from_lang: u32,
  pub to_lang_li: Vec<u32>,
}

impl FromTo {
  pub fn to_lang_li(&self) -> Vec<u32> {
    if self.to_lang_li.is_empty() {
      lang::LANG
        .into_iter()
        .map(|i| *i)
        .filter(|l| *l != self.from_lang)
        .collect()
    } else {
      self.to_lang_li.clone()
    }
  }
}
