use std::{collections::HashMap, path::Path};

use lang::{Lang, LANG_CODE};
use roaring::RoaringBitmap;

#[derive(Debug, Default)]
pub struct LangLi {
  pub lang: RoaringBitmap,
  pub li: crate::Li,
}

#[derive(Debug, Default)]
pub struct LangTree(pub Vec<LangLi>);

impl LangTree {
  pub fn rel_lang_set(self, root: impl AsRef<Path>) -> std::io::Result<RelLangSet> {
    let root = root.as_ref();
    let mut r = HashMap::new();
    for i in self.0 {
      for rel in i.li.iter() {
        let mut lang_set = RoaringBitmap::new();
        for lang in &i.lang {
          if std::fs::exists(root.join(LANG_CODE[lang as usize]).join(&rel))? {
            lang_set.push(lang);
          }
        }
        if !lang_set.is_empty() {
          r.insert(rel.to_owned(), lang_set);
        }
      }
    }
    Ok(RelLangSet(r))
  }
}

pub fn loads(iter: impl IntoIterator<Item = String>) -> LangTree {
  let mut r = vec![];
  let mut buf = String::new();
  let mut lang = None;
  for i in iter {
    if i.starts_with("@") {
      if let Some(lang) = lang {
        if let Ok(li) = xerr::ok!(serde_yaml::from_str::<crate::Li>(&buf)) {
          r.push(LangLi { lang, li });
        }
      }
      buf = String::new();
      if let Ok(l) = xerr::ok!(burl::d(&i[1..])) {
        if let Ok(l) = xerr::ok!(vb::diffd(&l)) {
          let mut t = RoaringBitmap::new();
          for i in l {
            t.insert(i as u32);
          }
          lang = Some(t);
          continue;
        }
      }
      lang = None;
    } else {
      buf += &i;
      buf.push('\n');
    }
  }

  if let Some(lang) = lang {
    if !buf.is_empty() {
      if let Ok(li) = xerr::ok!(serde_yaml::from_str::<crate::Li>(&buf)) {
        r.push(LangLi { lang, li });
      }
    }
  }
  LangTree(r)
}

pub fn dumps(iter: impl IntoIterator<Item = (Vec<u8>, crate::Li)>) -> String {
  let mut r = String::new();
  for (lang, li) in iter {
    if let Ok(yml) = xerr::ok!(serde_yaml::to_string(&li)) {
      r += "@";
      r += &burl::e(lang);
      r.push('\n');
      r += &yml;
    }
  }
  r
}

#[derive(Debug, Default, Clone)]
pub struct RelLangSet(pub HashMap<String, RoaringBitmap>);

pub fn lang_li_e(lang_li: &RoaringBitmap) -> Vec<u8> {
  vb::diffe(lang_li.iter().map(|i| i as u64).collect::<Vec<_>>())
}

impl RelLangSet {
  pub fn contains(&self, lang: Lang, rel: impl AsRef<str>) -> bool {
    let rel = rel.as_ref();
    if let Some(lang_set) = self.0.get(rel) {
      lang_set.contains(lang as u32)
    } else {
      false
    }
  }

  pub fn insert(&mut self, lang: Lang, rel: impl AsRef<str>) -> bool {
    let rel = rel.as_ref();
    if let Some(lang_set) = self.0.get_mut(rel) {
      lang_set.insert(lang as u32);
      return true;
    } else {
      self
        .0
        .insert(rel.to_owned(), RoaringBitmap::from_iter([lang as u32]));
    }
    false
  }

  pub fn remove(&mut self, lang: Lang, rel: impl AsRef<str>) -> bool {
    let rel = rel.as_ref();
    if let Some(lang_set) = self.0.get_mut(rel) {
      if lang_set.remove(lang as u32) {
        if lang_set.is_empty() {
          self.0.remove(rel);
        }
        return true;
      }
    }
    false
  }

  pub fn dumps(self) -> String {
    let mut lang_rel = HashMap::new();
    for (rel, lang) in &self.0 {
      let lang = lang_li_e(lang);
      lang_rel.entry(lang).or_insert_with(Vec::new).push(rel);
    }

    let mut lang_rel = lang_rel
      .into_iter()
      .map(|(lang, mut rel_li)| {
        rel_li.sort();
        (lang, crate::Li::from_iter(rel_li))
      })
      .collect::<Vec<_>>();

    lang_rel.sort_by(|a, b| a.0.cmp(&b.0));

    dumps(lang_rel.into_iter())
  }
}
