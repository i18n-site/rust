use lang::{Lang, LANG_CODE};
use roaring::RoaringBitmap;

#[derive(Debug)]
pub struct LangLi {
  pub lang: RoaringBitmap,
  pub li: crate::Li,
}

pub struct LangTree(pub Vec<LangLi>);

impl LangTree {
  pub fn iter(&self) -> impl IntoIterator<Item = String> + use<'_> {
    self
      .0
      .iter()
      .map(|i| {
        i.lang
          .iter()
          .map(|lang| {
            i.li
              .iter()
              .map(move |i| format!("{}/{i}", LANG_CODE[lang as usize]))
          })
          .flatten()
      })
      .flatten()
  }

  pub fn contains(&self, lang: Lang, rel: impl AsRef<str>) -> bool {
    for i in &self.0 {
      if i.lang.contains(lang as u32) && i.li.contains(rel.as_ref()) {
        return true;
      }
    }
    false
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
        let mut t = RoaringBitmap::new();
        for i in l {
          t.insert(i as u32);
        }
        lang = Some(t);
        continue;
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

pub fn dumps(iter: impl IntoIterator<Item = (Vec<Lang>, crate::Li)>) -> String {
  let mut r = String::new();
  for (mut lang_li, li) in iter {
    lang_li.sort();
    r += "@";
    if let Ok(yml) = xerr::ok!(serde_yaml::to_string(&li)) {
      r += &burl::e(vb::diffe(
        lang_li.into_iter().map(|i| i as u64).collect::<Vec<_>>(),
      ));
      r.push('\n');
      r += &yml;
    }
  }
  r
}
