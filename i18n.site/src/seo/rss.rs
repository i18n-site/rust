use gxhash::{HashMap, HashMapExt};
use lang::Lang;
use ytree::sitemap::Sitemap;

use super::Seo;

pub struct Rss {
  pub exist: Sitemap,
  pub li: HashMap<Lang, Vec<(String, String, String)>>,
}

fn convert_rel_lang_set(sitemap: &Sitemap) -> HashMap<u32, HashMap<String, u64>> {
  let mut result: HashMap<u32, HashMap<String, u64>> = HashMap::new();

  for (key, lang_set) in &sitemap.rel_lang_set {
    for value in lang_set.lang_set.iter() {
      result
        .entry(value)
        .or_insert_with(HashMap::new)
        .insert(key.clone(), lang_set.ts);
    }
  }

  result
}

impl Rss {
  pub fn new(exist: Sitemap) -> Self {
    Self {
      exist,
      li: Default::default(),
    }
  }

  pub fn gen(&self, host: &str) -> impl IntoIterator<Item = (String, String)> + use<'_> {
    let mut lang_rel_ts = convert_rel_lang_set(&self.exist);
    self
      .li
      .iter()
      .filter_map(move |(lang, rel_title_htm)| {
        if let Some(mut rel_ts) = lang_rel_ts.remove(&(*lang as u32)) {
          let mut limit = 10;
          let mut li = Vec::with_capacity(limit);
          for (rel, title, htm) in rel_title_htm {
            if limit == 0 {
              break;
            }
            if let Some(ts) = rel_ts.remove(rel) {
              limit -= 1;
              dbg!((lang, rel, ts));
            }
          }
          if limit > 0 {}
          Some(li)
        } else {
          None
        }
      })
      .flatten()
  }

  pub fn push(
    &mut self,
    lang: Lang,
    rel: impl Into<String>,
    title: impl Into<String>,
    htm: impl Into<String>,
  ) {
    let rel = rel.into();

    if !self.exist.contains(lang, &rel) {
      let title = title.into();
      let htm = htm.into();
      self.li.entry(lang).or_default().push((rel, title, htm));
    }
  }
}
