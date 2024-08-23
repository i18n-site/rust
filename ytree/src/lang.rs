use std::{collections::HashMap, path::Path};

use lang::{Lang, LANG_CODE};
use roaring::RoaringBitmap;

#[derive(Debug, Default)]
pub struct LangLi {
  pub lang: Vec<u64>,
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
        let mut rel = rel.rsplit("#");

        if let Some(ts) = rel.next() {
          if let Ok(ts) = burl::d(ts)
            && let Some(rel) = rel.remainder()
          {
            let ts = intbin::bin_u64(ts);
            let mut lang_set = RoaringBitmap::new();
            for lang in &i.lang {
              let lang = *lang;
              if std::fs::exists(root.join(LANG_CODE[lang as usize]).join(rel))? {
                lang_set.push(lang as u32);
              }
            }
            if !lang_set.is_empty() {
              r.insert(rel.to_owned(), LangSet { ts, lang_set });
            }
          }
        }
      }
    }
    Ok(RelLangSet {
      rel_lang_set: r,
      now: sts::sec(),
    })
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
          lang = Some(l);
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
pub struct LangSet {
  pub ts: u64,
  pub lang_set: RoaringBitmap,
}

#[derive(Debug, Default, Clone)]
pub struct RelLangSet {
  pub rel_lang_set: HashMap<String, LangSet>,
  pub now: u64,
}

pub fn lang_li_e(lang_li: &RoaringBitmap) -> Vec<u8> {
  vb::diffe(lang_li.iter().map(|i| i as u64).collect::<Vec<_>>())
}

impl RelLangSet {
  pub fn contains(&self, lang: Lang, rel: impl AsRef<str>) -> bool {
    let rel = rel.as_ref();
    if let Some(t) = self.rel_lang_set.get(rel) {
      t.lang_set.contains(lang as u32)
    } else {
      false
    }
  }

  pub fn insert(&mut self, lang: Lang, rel: impl AsRef<str>) -> bool {
    let rel = rel.as_ref();
    if let Some(t) = self.rel_lang_set.get_mut(rel) {
      t.lang_set.insert(lang as u32);
      t.ts = self.now;
      return true;
    } else {
      self.rel_lang_set.insert(
        rel.to_owned(),
        LangSet {
          ts: self.now,
          lang_set: RoaringBitmap::from_iter([lang as u32]),
        },
      );
    }
    false
  }

  pub fn remove(&mut self, lang: Lang, rel: impl AsRef<str>) -> bool {
    let rel = rel.as_ref();
    if let Some(r) = self.rel_lang_set.get_mut(rel) {
      let lang_set = &mut r.lang_set;
      if lang_set.remove(lang as u32) {
        if lang_set.is_empty() {
          self.rel_lang_set.remove(rel);
        }
        return true;
      }
    }
    false
  }

  pub fn dumps(self) -> String {
    let mut lang_rel = HashMap::new();
    for (rel, t) in &self.rel_lang_set {
      let lang = lang_li_e(&t.lang_set);
      let rel = format!("{}#{}", rel, burl::e(intbin::u64_bin(t.ts)));
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
