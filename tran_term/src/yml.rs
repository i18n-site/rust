use std::collections::HashMap;

use aok::Result;
use saphyr::{LoadableYamlNode, Yaml, Yaml::Mapping};

#[derive(PartialEq, Default)]
pub struct Term {
  pub global: Option<crate::Term>,
  pub lang: HashMap<u32, crate::Term>,
}

impl Term {
  pub fn restore(&mut self, lang: u32, txt: impl Into<String>) -> String {
    let mut txt = txt.into();

    if let Some(ref mut map) = self.lang.get_mut(&lang) {
      txt = map.restore(&txt);
    } else if let Some(ref mut global) = self.global {
      txt = global.restore(&txt);
    }

    txt
  }

  pub fn replace(&mut self, lang: u32, txt: impl Into<String>) -> String {
    let mut txt = txt.into();

    if let Some(ref mut map) = self.lang.get_mut(&lang) {
      if let Some(t) = map.replace(&txt) {
        txt = t;
      }
    } else if let Some(ref mut global) = self.global
      && let Some(t) = global.replace(&txt)
    {
      txt = t;
    }

    txt
  }
}

fn map_li(map: &Yaml) -> Vec<(String, String)> {
  if let Mapping(map) = map {
    let mut li = Vec::with_capacity(map.len());
    for (k, v) in map {
      if let Some(k) = k.as_str() {
        li.push((k.to_owned(), v.as_str().unwrap_or_default().to_owned()))
      }
    }
    li
  } else {
    vec![]
  }
}

pub fn load(yml: impl AsRef<str>) -> Result<Term> {
  let mut term = Term::default();
  let mut lang_li = Vec::new();
  let yml = yml.as_ref();
  if yml.is_empty() {
    return Ok(term);
  }
  for i in Yaml::load_from_str(yml)? {
    if let Mapping(ref m) = i {
      for (lang, term_yml) in m {
        let term_li = map_li(term_yml);

        if !term_li.is_empty() {
          if lang.is_null() {
            term.global = Some(crate::Term::load(term_li)?);
          } else if let Some(lang) = lang.as_str()
            && let Some(lang) = lang::by_str(lang)
          {
            lang_li.push((lang as u32, term_li));
          }
        }
      }
    }
  }

  for (lang, term_li) in lang_li {
    term.lang.insert(
      lang,
      if let Some(ref global) = term.global {
        crate::Term::load(
          term_li
            .into_iter()
            .chain(global.map.iter().map(|(k, v)| (k.clone(), v.clone()))),
        )
      } else {
        crate::Term::load(term_li)
      }?,
    );
  }

  Ok(term)
}
