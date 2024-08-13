use gxhash::HashMap;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct I18n {
  pub fromTo: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Upload {
  pub ext: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nav {
  pub i18n: String,
  pub r#use: String,
  pub url: Option<String>,
  pub menu: Option<String>,
  pub arg: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
  pub i18n: i18_conf::I18nConf,
  pub ignore: Option<Vec<String>>,
  pub nav: Vec<Nav>,
  pub upload: Upload,
}

// #[allow(non_snake_case)]
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Site {
//   pub ver: Option<String>,
// }

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HtmConf {
  pub api: Option<String>,
  // pub site: Option<Site>,
  // pub outdir: Option<String>,
  pub importmap: HashMap<String, String>,
  pub v: String,
  pub x: String,
}
