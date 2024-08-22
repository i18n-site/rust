use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct S3 {
  pub endpoint: String,
  pub region: Option<String>,
  pub ak: String,
  pub sk: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Conf {
  pub token: Option<String>,
  pub s3: Option<HashMap<String, Vec<S3>>>,
}

#[static_init::dynamic]
pub static CONF: Conf = {
  let yml = ifs::confdir().join("i18n.site.yml");
  if let Ok(r) = xerr::ok!(ifs::r(yml)) {
    if !r.is_empty() {
      if let Ok::<Conf, _>(c) = serde_yaml::from_slice(&r) {
        return c;
      }
    }
  }
  Default::default()
};

pub fn token() -> Option<String> {
  if let Ok(token) = std::env::var("I18N_SITE_TOKEN") {
    if !token.is_empty() {
      return Some(token);
    }
  }
  CONF.token.clone()
}
