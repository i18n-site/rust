use std::{collections::HashMap, path::Path};

use aok::Result;
use lang::Lang;
use serde::{Deserialize, Serialize};

use crate::api;

#[derive(Debug, Serialize, Deserialize)]
pub struct Term(HashMap<String, HashMap<String, String>>);

pub fn term(workdir: &Path) -> Result<Vec<api::Term>> {
  let mut term_li = Vec::new();

  let term_yml = workdir.join(".i18n/term.yml");
  if term_yml.exists() {
    let term_yml = ifs::rstr(term_yml)?;
    let term: Term = serde_yaml::from_str(&term_yml)?;
    for (lang_li, map) in term.0 {
      let mut iter = lang_li.split('>');
      if let Some(from_lang) = iter.next() {
        if let Ok::<Lang, _>(from_lang) = xerr::ok!(from_lang.try_into()) {
          let to_lang_str_li = iter.collect::<Vec<_>>();
          let to_lang_str_li_is_empty = to_lang_str_li.is_empty();

          let mut to_lang_li = Vec::with_capacity(to_lang_str_li.len());

          for lang in to_lang_str_li {
            if let Ok::<Lang, _>(to_lang) = xerr::ok!(lang.try_into()) {
              to_lang_li.push(to_lang as _);
            }
          }

          if to_lang_li.is_empty() && !to_lang_str_li_is_empty {
            continue;
          }

          term_li.push(api::Term {
            from_lang: from_lang as u32,
            to_lang_li,
            map,
          });
        }
      }
    }
  }

  Ok(term_li)
}
