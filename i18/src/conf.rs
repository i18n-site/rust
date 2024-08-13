use i18_conf::I18nConf;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
  pub ignore: Option<Vec<String>>,
  pub i18n: I18nConf,
}
