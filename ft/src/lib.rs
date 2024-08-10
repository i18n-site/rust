#![feature(min_specialization)]

use gxhash::{HashMap, HashSet, HashSetExt};
use lang::{IntoEnumIterator, Lang};
use strum::EnumCount;

#[derive(Default, Debug)]
pub struct FromTo {
  pub tf: HashMap<Lang, Lang>,
  pub ft: HashMap<Lang, Vec<Lang>>,
  pub default_from: Option<Lang>,
}

// impl From<Vec<(u16, Vec<u16>)>> for FromTo {
impl<
    F: TryInto<Lang>,
    T: TryInto<Lang>,
    I: IntoIterator<Item = T>,
    V: IntoIterator<Item = (F, I)>,
  > From<V> for FromTo
where
  <F as TryInto<Lang>>::Error: std::fmt::Display,
  <T as TryInto<Lang>>::Error: std::fmt::Display,
{
  default fn from(from_to: V) -> Self {
    let mut ft = FromTo::default();
    let mut exist = HashSet::new();
    for (from_lang, to_li) in from_to {
      if let Ok(from_lang) = xerr::ok!(from_lang.try_into()) {
        // 避免成环
        let from_src = {
          let mut from_src = HashSet::new();
          let mut lang = from_lang;
          while let Some(src_lang) = ft.from(lang) {
            from_src.insert(src_lang);
            lang = src_lang;
          }
          from_src
        };
        let mut to_count = 0;

        let mut li = Vec::new();
        for to_lang in to_li {
          to_count += 1;
          if let Ok::<Lang, _>(to_lang) = xerr::ok!(to_lang.try_into()) {
            if to_lang == from_lang || from_src.contains(&to_lang) {
              continue;
            }

            // 避免一个 to 有多个 from
            if !exist.contains(&to_lang) {
              exist.insert(to_lang);
              li.push(to_lang);
              ft.tf.insert(to_lang, from_lang);
            }
          }
        }
        if li.is_empty() {
          if to_count == 0 {
            ft.default_from = Some(from_lang)
          }
        } else {
          li.sort();
          ft.ft.insert(from_lang, li);
        }
      }
    }
    ft
  }
}

impl<'a> std::iter::FromIterator<(&'a String, &'a String)> for FromTo {
  fn from_iter<T: IntoIterator<Item = (&'a String, &'a String)>>(iter: T) -> Self {
    iter
      .into_iter()
      .map(|(k, v)| (k, v.split_whitespace()))
      .into()
  }
}

impl FromTo {
  pub fn root(&self) -> Option<Lang> {
    let mut exist = HashSet::new();

    if let Some(mut l) = self.default_from {
      let mut r = None;
      while let Some(i) = self.tf.get(&l) {
        if exist.contains(i) {
          break;
        }
        l = *i;
        exist.insert(l);
        r = Some(l);
      }
      if r.is_some() {
        return r;
      }
      return self.default_from;
    }

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

  pub fn to_lang_count(&self, lang: Lang) -> usize {
    if let Some(li) = self.to_li(lang) {
      let mut r = HashSet::with_capacity(li.len());
      let mut li = Vec::from(&li[..]);
      while let Some(t) = li.pop() {
        r.insert(t);
        if let Some(l2) = self.to_li(t) {
          for j in &l2[..] {
            if !r.contains(j) {
              li.push(*j);
              r.insert(*j);
            }
          }
        }
      }
      return r.len();
    }
    0
  }
  pub fn to_li_recursive<T: TryInto<Lang>>(&self, lang: T) -> HashSet<Lang>
  where
    <T as TryInto<Lang>>::Error: std::fmt::Display,
  {
    if let Some(li) = self.to_li(lang) {
      let mut r = HashSet::default();
      for i in li {
        r.insert(i);
        if let Some(to_li) = self.to_li(i) {
          for i in to_li {
            if !r.contains(&i) {
              r.insert(i);
              r.extend(self.to_li_recursive(i));
            }
          }
        }
      }
      r
    } else {
      Default::default()
    }
  }

  pub fn to_li<T: TryInto<Lang>>(&self, lang: T) -> Option<Vec<Lang>>
  where
    <T as TryInto<Lang>>::Error: std::fmt::Display,
  {
    if let Ok(lang) = xerr::ok!(lang.try_into()) {
      if let Some(r) = self.ft.get(&lang) {
        return Some(r.clone());
      }
      if self.default_from == Some(lang) {
        let mut r = Vec::with_capacity(Lang::COUNT - self.tf.len());
        for i in Lang::iter() {
          if i != lang && !self.tf.contains_key(&i) && !self.ft.contains_key(&i) {
            r.push(i);
          }
        }
        if !r.is_empty() {
          return Some(r);
        }
      }
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

  pub fn root_all_lang_li(&self) -> Vec<Lang> {
    let root = self.root();
    let mut r = self.all_lang_set();
    if let Some(root) = root {
      r.remove(&root);
    }
    let mut r = r.into_iter().collect::<Vec<_>>();
    r.sort();
    if let Some(root) = root {
      r.insert(0, root);
    }
    r
  }

  pub fn all_lang_li(&self) -> Vec<Lang> {
    let mut r = self.all_lang_set().into_iter().collect::<Vec<_>>();
    r.sort();
    r
  }

  pub fn all_lang_set(&self) -> HashSet<Lang> {
    let mut r = HashSet::new();
    for i in self.from_lang_li() {
      r.insert(i);
      if let Some(li) = self.to_li(i) {
        for j in &li[..] {
          r.insert(*j);
        }
      }
    }
    r
  }

  pub fn from_lang_li(&self) -> Vec<Lang> {
    let mut r = self.ft.keys().copied().collect::<Vec<_>>();
    if let Some(default_from) = self.default_from {
      r.push(default_from);
    }
    r
  }

  // pub fn add(&mut self, from: u16, to_set: HashSet<u16>) {
  //   if let Ok(from) = from.try_into() {
  //     if to_set.is_empty() {
  //       self.default_from = Some(from);
  //     } else {
  //       let mut to_li: Vec<_> = to_set
  //         .into_iter()
  //         .filter_map(|i| {
  //           if let Ok(i) = i.try_into() {
  //             self.tf.insert(i, from);
  //             Some(i)
  //           } else {
  //             None
  //           }
  //         })
  //         .collect();
  //       to_li.sort();
  //       self.ft.insert(from, to_li);
  //     }
  //   }
  // }
}
