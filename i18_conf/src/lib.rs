use globset::{Glob, GlobSet, GlobSetBuilder};
use gxhash::HashMap;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct FromTo {
  pub fromTo: HashMap<String, String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct DirI18nConf {
  pub fromTo: HashMap<String, String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct I18nConf {
  pub fromTo: HashMap<String, String>,
  pub path: Option<HashMap<String, DirI18nConf>>,
}

pub fn build_ignore(ignore: &[String]) -> GlobSet {
  let mut builder = GlobSetBuilder::new();
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
  builder.add(Glob::new(".*").unwrap());
  builder.build().unwrap()
}
