use std::collections::{HashMap, HashSet};

use lang::{IntoEnumIterator, Lang};
use strum::EnumCount;

#[derive(Default, Debug)]
pub struct FromTo {
  pub tf: HashMap<Lang, Lang>,
  pub ft: HashMap<Lang, Vec<Lang>>,
  pub default_from: Option<Lang>,

  #[cfg(feature = "str")]
  pub lang_str: HashMap<Lang, String>,
}

#[cfg(feature = "str")]
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

impl FromTo {
  pub fn root(&self) -> Option<Lang> {
    if self.default_from.is_some() {
      return self.default_from;
    }
    let mut exist = HashSet::new();
    if let Some(i) = self.ft.keys().next() {
      let mut i = *i;
      loop {
        let from = self.from(i);
        if let Some(t) = from {
          if exist.contains(&t) {
            break;
          }
          exist.insert(t);
          i = t;
        } else {
          break;
        }
      }
      return Some(i);
    }
    None
  }

  pub fn has_to_li(&self, lang: Lang) -> bool {
    if self.ft.contains_key(&lang) {
      return true;
    }
    Some(lang) == self.default_from
  }

  pub fn to_li(&self, lang: Lang) -> Option<Box<[Lang]>> {
    if let Some(r) = self.ft.get(&lang) {
      return Some(Box::from(&r[..]));
    }
    if let Some(lang) = self.default_from {
      let mut r = Vec::with_capacity(Lang::COUNT - self.tf.len());
      for i in Lang::iter() {
        if i != lang && !self.tf.contains_key(&i) && !self.ft.contains_key(&i) {
          r.push(i);
        }
      }
      return Some(r.into());
    }
    None
  }

  pub fn from<T: TryInto<Lang>>(&self, lang: T) -> Option<Lang>
  where
    <T as TryInto<Lang>>::Error: std::fmt::Display,
  {
    if let Ok(lang) = xerr::ok!(lang.try_into()) {
      return match self.tf.get(&lang).map(|i| Some(*i)) {
        Some(r) => r,
        None => {
          if let Some(default_from) = self.default_from {
            if lang != default_from && self.tf.get(&default_from) != Some(&lang) {
              return Some(default_from);
            }
          }
          None
        }
      };
    }
    None
  }

  pub fn add(&mut self, from: u16, to_set: HashSet<u16>) {
    if let Ok(from) = from.try_into() {
      if to_set.is_empty() {
        self.default_from = Some(from);
      } else {
        let to_li = to_set
          .into_iter()
          .filter_map(|i| {
            if let Ok(i) = i.try_into() {
              self.tf.insert(i, from);
              Some(i)
            } else {
              None
            }
          })
          .collect();
        self.ft.insert(from, to_li);
      }
    }
  }
}
