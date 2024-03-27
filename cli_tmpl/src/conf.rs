use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct I18n {
  pub fromTo: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Render(HashMap<String, Vec<String>>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Nav(HashMap<String, Option<String>>);

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
  pub host: String,
  pub i18n: I18n,
  pub render: Render,
  pub nav: Nav,
  pub uploadExt: Vec<String>,
}
