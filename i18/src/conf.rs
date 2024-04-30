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

#[derive(Debug)]
pub struct Config {
  pub i18n: I18nConf,
  pub ignore: GlobSet,
}

impl From<Conf> for Config {
  fn from(conf: Conf) -> Self {
    Self {
      i18n: conf.i18n,
      ignore: {
        let mut builder = GlobSetBuilder::new();
        if let Some(ignore) = conf.ignore {
          ignore.into_iter().for_each(|regex| {
            builder.add(Glob::new(&regex).unwrap());
          });
        }

        builder.build().unwrap()
      },
    }
  }
}
