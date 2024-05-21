use std::collections::HashMap;

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

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
  pub id: String,
  pub i18n: i18::I18nConf,
  pub ignore: Option<Vec<String>>,
  // pub nav: Vec<String>,
  pub nav: Vec<HashMap<String, String>>,
  pub upload: Upload,
}
