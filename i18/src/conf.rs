use std::collections::HashMap;

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
}
