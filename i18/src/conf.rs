use std::collections::HashMap;

use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct FromTo {
  pub fromTo: HashMap<String, String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct I18nConf {
  pub fromTo: HashMap<String, String>,
  pub yml: Option<FromTo>,
  pub md: Option<FromTo>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
  pub i18n: I18nConf,
  pub ignore: Option<Vec<String>>,
}

pub fn build_ignore(ignore: &Option<Vec<String>>) -> GlobSet {
  let mut builder = GlobSetBuilder::new();
  if let Some(ignore) = ignore {
    ignore.iter().for_each(|regex| {
      let glob = if regex.starts_with("/") {
        Glob::new(regex)
      } else {
        Glob::new(&(String::from("/**/") + regex))
      };

      if let Ok(glob) = xerr::ok!(glob) {
        builder.add(glob);
      }
    });
  }
  builder.add(Glob::new(".*").unwrap());

  builder.build().unwrap()
}
