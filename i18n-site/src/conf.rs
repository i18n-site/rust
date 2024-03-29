use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct I18n {
  pub fromTo: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Render(pub HashMap<String, Vec<String>>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Nav(pub HashMap<String, Option<String>>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Upload {
  pub ext: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
  pub host: String,
  pub i18n: i18::I18nConf,
  pub render: Render,
  pub nav: Nav,
  pub upload: Upload,
}
