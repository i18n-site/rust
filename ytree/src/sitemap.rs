use std::{
  collections::{HashMap, HashSet},
  path::Path,
};

use lang::{CODE, Lang};
use roaring::RoaringBitmap;

#[derive(Debug, Default)]
pub struct LangLi {
  pub lang: Vec<u64>,
  pub li: crate::Li,
}

#[derive(Debug, Default)]
pub struct LangTree(pub Vec<LangLi>);

#[derive(Debug, Default, Clone)]
pub struct Sitemap {
  pub rel_lang_set: HashMap<String, LangSet>,
  pub now: u64,
}

#[derive(Debug, Default, Clone)]
pub struct LangRelTs {
  pub lang_rel: HashMap<Lang, HashSet<String>>,
  pub rel_ts: HashMap<String, u64>,
}

pub fn loads(iter: impl IntoIterator<Item = String>) -> LangTree {
  let mut r = vec![];
  let mut buf = String::new();
  let mut lang = None;
  for i in iter {
    let i = i.trim_end();
    if let Some(i) = i.chars().next() {
      if "<>#".contains(i) {
        continue;
      }
    } else {
      continue;
    }

    if let Some(i) = i.strip_prefix("@") {
      if let Some(lang) = lang
        && let Ok(li) = xerr::ok!(serde_yaml::from_str::<crate::Li>(&buf))
      {
        r.push(LangLi { lang, li });
      }
      buf = String::new();
      if let Ok(l) = xerr::ok!(burl::d(i))
        && let Ok(l) = xerr::ok!(vb::diffd(&l))
      {
        lang = Some(l);
        continue;
      }
      lang = None;
    } else {
      buf += i;
      buf.push('\n');
    }
  }

  if let Some(lang) = lang
    && !buf.is_empty()
    && let Ok(li) = xerr::ok!(serde_yaml::from_str::<crate::Li>(&buf))
  {
    r.push(LangLi { lang, li });
  }
  LangTree(r)
}

pub fn dumps(lang_rel: HashMap<Vec<u8>, Vec<String>>) -> String {
  let mut lang_rel = lang_rel
    .into_iter()
    .map(|(lang, rel_li)| (lang, crate::Li::from_vec(rel_li)))
    .collect::<Vec<_>>();
  lang_rel.sort_by(|a, b| a.0.cmp(&b.0));
  let mut r = String::new();
  for (lang, li) in lang_rel {
    if let Ok(yml) = xerr::ok!(serde_yaml::to_string(&li)) {
      r += "@";
      r += &burl::e(lang);
      r.push('\n');
      r += &yml;
    }
  }
  r
}

pub fn lang_li_e(lang_li: &RoaringBitmap) -> Vec<u8> {
  vb::diffe(lang_li.iter().map(|i| i as u64).collect::<Vec<_>>())
}

impl LangRelTs {
  pub fn contains(&self, lang: Lang, rel: impl AsRef<str>) -> bool {
    let rel = rel.as_ref();
    if let Some(t) = self.lang_rel.get(&lang) {
      t.contains(rel)
    } else {
      false
    }
  }

  pub fn dumps(&self) -> String {
    let mut rel_lang_set: HashMap<String, RoaringBitmap> = HashMap::new();

    for (lang, rel_set) in &self.lang_rel {
      for rel in rel_set {
        rel_lang_set
          .entry(rel.into())
          .or_default()
          .insert(*lang as u32);
      }
    }

    let mut lang_rel = HashMap::new();
    for (rel, lang_set) in rel_lang_set {
      let lang = lang_li_e(&lang_set);
      lang_rel.entry(lang).or_insert_with(Vec::new).push(format!(
        "{}#{}",
        rel,
        burl::e(intbin::to_bin(self.rel_ts[rel.as_str()]))
      ));
    }

    dumps(lang_rel)
  }
}

impl LangTree {
  pub fn lang_rel_ts(self) -> LangRelTs {
    let mut lang_rel: HashMap<Lang, HashSet<String>> = HashMap::new();
    let mut rel_ts = HashMap::with_capacity(self.0.len());
    for i in self.0 {
      for rel in i.li.iter() {
        let mut rel = rel.rsplit("#");

        if let Some(ts) = rel.next()
          && let Ok(ts) = burl::d(ts)
          && let Some(rel) = rel.remainder()
        {
          let ts = intbin::bin_u64(ts);
          rel_ts.insert(rel.into(), ts);
          for lang in &i.lang {
            let lang = *lang;
            if let Ok(lang) = (lang as u16).try_into() {
              lang_rel.entry(lang).or_default().insert(rel.into());
            }
          }
        }
      }
    }
    LangRelTs { lang_rel, rel_ts }
  }

  pub fn sitemap(self, root: impl AsRef<Path>) -> std::io::Result<Sitemap> {
    let root = root.as_ref();
    let mut r = HashMap::new();
    for i in self.0 {
      for rel in i.li.iter() {
        let mut rel = rel.rsplit("#");

        if let Some(ts) = rel.next()
          && let Ok(ts) = burl::d(ts)
          && let Some(rel) = rel.remainder()
        {
          let ts = intbin::bin_u64(ts);
          let mut lang_set = RoaringBitmap::new();
          for lang in &i.lang {
            let lang = *lang;
            if std::fs::exists(root.join(CODE[lang as usize]).join(rel))? {
              let _ = lang_set.try_push(lang as u32);
            }
          }
          if !lang_set.is_empty() {
            r.insert(rel.to_owned(), LangSet { ts, lang_set });
          }
        }
      }
    }
    Ok(Sitemap {
      rel_lang_set: r,
      now: sts::sec(),
    })
  }
}

#[derive(Debug, Default, Clone)]
pub struct LangSet {
  pub ts: u64,
  pub lang_set: RoaringBitmap,
}

/*
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">
<url>
<loc>https://www.example.com/page1</loc>
<lastmod>2023-04-30T12:34:56</lastmod>
<xhtml:link rel="alternate" hreflang="en" href="https://www.example.com/page1" />
<xhtml:link rel="alternate" hreflang="fr" href="https://www.example.com/fr/page1" />
<xhtml:link rel="alternate" hreflang="es" href="https://www.example.com/es/page1" />
</url>
</urlset>

*/

pub fn rel_url(rel: &str) -> &str {
  if let Some(rel) = rel.strip_suffix("/README") {
    rel
  } else if rel == "README" {
    "/"
  } else {
    rel
  }
}

pub fn md_url(rel: &str) -> &str {
  if let Some(rel) = rel.strip_suffix(".md") {
    rel_url(rel)
  } else {
    rel
  }
}

pub const XML_HEAD: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
pub const XML_URLSET_BEGIN: &str = const_str::concat!(
  XML_HEAD,
  r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">"#
);
pub const XML_URLSET_END: &str = "</urlset>";
pub const XML_URLSET_LEN: usize = XML_URLSET_BEGIN.len() + XML_URLSET_END.len();
pub const MAX_SIZE: usize = 10 * (2 << 19) - (2 << 16);
pub const MAX_URL: u64 = 49000;

pub fn generate(xml: &str) -> Vec<u8> {
  let xml = format!("{XML_URLSET_BEGIN}{xml}{XML_URLSET_END}");
  xml.into_bytes()
}

impl Sitemap {
  pub fn generate<'a>(&'a self, host: &'a str) -> impl IntoIterator<Item = Vec<u8>> + 'a {
    let mut iter = self.rel_lang_set.iter();
    let mut n = 0;
    let mut xml = String::new();
    let mut len = XML_URLSET_LEN;

    std::iter::from_fn(move || {
      for (rel, t) in iter.by_ref() {
        let dt = tsfmt::utc(t.ts);
        let url = md_url(rel);

        let urlxml = format!(
            "<url><loc>https://{host}/{url}</loc><lastmod>{dt}+00:00</lastmod>{}</url>",
            t.lang_set
            .iter()
            .map(|lang| {
              let lang = CODE[lang as usize];
              format!(
                r#"<xhtml:link rel="alternate" hreflang="{lang}" href="https://{host}/{lang}/{url}.htm"/>"#
              )
            })
            .collect::<Vec<_>>()
            .join("")
          );

        let t_n = t.lang_set.len() + 1;
        let next_n = n + t_n;
        let next_len = len + urlxml.len();
        if next_len > MAX_SIZE || next_n > MAX_URL {
          let result = generate(&xml);
          xml = urlxml;
          len = xml.len() + XML_URLSET_LEN;
          n = t_n;
          return Some(result);
        }
        n = next_n;
        len = next_len;
        xml += &urlxml;
      }

      if xml.is_empty() {
        None
      } else {
        let result = generate(&xml);
        xml.clear(); // for end iter
        Some(result)
      }
    })
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

  pub fn set(&self) -> HashSet<(Lang, String)> {
    let mut r = HashSet::new();
    for (rel, lang_li) in &self.rel_lang_set {
      for lang in &lang_li.lang_set {
        if let Ok(lang) = (lang as u16).try_into() {
          r.insert((lang, rel.clone()));
        }
      }
    }
    r
  }

  pub fn dumps(&self) -> String {
    let mut lang_rel = HashMap::new();
    for (rel, t) in &self.rel_lang_set {
      let lang = lang_li_e(&t.lang_set);
      let rel = format!("{}#{}", rel, burl::e(intbin::to_bin(t.ts)));
      lang_rel.entry(lang).or_insert_with(Vec::new).push(rel);
    }
    dumps(lang_rel)
  }

  pub fn contains(&self, lang: Lang, rel: impl AsRef<str>) -> bool {
    let rel = rel.as_ref();
    if let Some(t) = self.rel_lang_set.get(rel) {
      t.lang_set.contains(lang as u32)
    } else {
      false
    }
  }
}
