use std::collections::HashMap;

use lang::Lang;

#[derive(Default, Debug)]
pub struct FromTo {
  pub ft: HashMap<Lang, Vec<Lang>>,
  pub tf: HashMap<Lang, Lang>,
  pub lang_str: HashMap<Lang, String>,
  pub default_from: Option<Lang>,
}

impl FromTo {
  pub fn from(&self, lang: impl TryInto<Lang>) -> Option<Lang> {
    if let Ok(lang) = lang.try_into() {
      let from = self.tf.get(&lang);
      if from.is_some() {
        return from.cloned();
      }
    }
    self.default_from
  }
}

impl From<&HashMap<String, String>> for FromTo {
  fn from(value: &HashMap<String, String>) -> Self {
    let len = value.len();
    let mut ft = HashMap::with_capacity(len);
    let mut lang_str = HashMap::with_capacity(len);
    let mut tf = HashMap::new();
    let mut default_from = None;
    for (kstr, vstr) in value {
      if let Ok::<Lang, _>(klang) = xerr::ok!(kstr.as_str().try_into()) {
        lang_str.insert(klang, kstr.into());
        let mut li = Vec::new();
        if vstr.is_empty() {
          default_from = Some(klang)
        } else {
          for i in vstr.split(' ') {
            if let Ok::<Lang, _>(vlang) = xerr::ok!(i.try_into()) {
              lang_str.insert(vlang, i.into());
              li.push(vlang);
              tf.insert(vlang, klang);
            }
          }
          if li.is_empty() {
            continue;
          }
        }
        ft.insert(klang, li);
      }
    }
    FromTo {
      ft,
      tf,
      lang_str,
      default_from,
    }
  }
}
