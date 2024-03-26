use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseConf {
  pub fromTo: Option<HashMap<String, String>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct I18nConf {
  pub fromTo: HashMap<String, String>,
  pub yml: Option<BaseConf>,
  pub md: Option<BaseConf>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
  pub i18n: Option<I18nConf>,
}
