use gxhash::HashMap;
use lang::Lang;

use crate::api;

/*
message Replace
{
  string prefix = 1;
  optional uint32 default_lang = 2;
  repeated uint32 lang_li = 3;
  map<uint32, uint32> lang_map = 4;
}
*/

pub fn parse(prefix: &str, conf: &str) -> Option<api::Replace> {
  let mut lang_li = vec![];
  let mut lang_map = std::collections::HashMap::default();
  let mut default_lang: Option<u32> = None;

  macro_rules! lang_err {
    ($lang:expr) => {
      tracing::error!("conf.yml i18n.replace {prefix} lang {} error", $lang);
    };
  }

  for i in conf.split_whitespace() {
    let mut li = i.split('>').collect::<Vec<&str>>();
    if li.len() == 1 {
      match <&str as TryInto<Lang>>::try_into(li[0]) {
        Ok(lang) => {
          lang_li.push(lang as u32);
        }
        Err(_) => {
          lang_err!(li[0]);
        }
      }
    } else {
      let to = li.pop().unwrap();
      match <&str as TryInto<Lang>>::try_into(to) {
        Ok(lang) => {
          let to = lang as u32;
          dbg!((li.len(), to, li[0]));
          if li.len() == 1 && li[0] == "" {
            default_lang = Some(to);
          } else {
            for i in li {
              match <&str as TryInto<Lang>>::try_into(i) {
                Ok(lang) => {
                  lang_map.insert(lang as u32, to);
                }
                Err(_) => {
                  lang_err!(i);
                }
              }
            }
          }
        }
        Err(_) => {
          lang_err!(to);
        }
      }
    }
  }

  if default_lang.is_none() && lang_li.is_empty() && lang_map.is_empty() {
    return None;
  }

  Some(api::Replace {
    prefix: prefix.into(),
    default_lang,
    lang_li,
    lang_map,
  })
}

pub fn replace(map: &HashMap<String, String>) -> Vec<api::Replace> {
  let mut li = vec![];
  for (prefix, val) in map {
    if let Some(r) = parse(prefix, val) {
      li.push(r);
    }
  }
  li
}
