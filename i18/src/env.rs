use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Conf {
  pub token: Option<String>,
}

#[static_init::dynamic]
pub static I18N_SITE_YML: Vec<u8> = {
  let yml = ifs::confdir().join("i18n.site.yml");
  ifs::r(yml).unwrap_or_default()
};

pub fn load<T: DeserializeOwned + Default>() -> serde_yaml::Result<T> {
  let yml = &*I18N_SITE_YML;
  if yml.is_empty() {
    return Ok(Default::default());
  }
  serde_yaml::from_slice(&I18N_SITE_YML)
}

#[static_init::dynamic]
pub static CONF: Conf = load().unwrap_or_default();

pub fn token() -> Option<String> {
  if let Ok(token) = std::env::var("I18N_SITE_TOKEN") {
    if !token.is_empty() {
      return Some(token);
    }
  }
  CONF.token.clone()
}
