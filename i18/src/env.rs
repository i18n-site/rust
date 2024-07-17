use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Env {
  pub token: Option<String>,
}

pub fn token() -> Option<String> {
  if let Ok(token) = std::env::var("I18N_SITE_TOKEN") {
    if !token.is_empty() {
      return Some(token);
    }
  }
  let yml = ifs::confdir().join("i18n.site.yml");
  if let Ok(r) = xerr::ok!(ifs::r(yml)) {
    if !r.is_empty() {
      if let Ok::<Env, _>(env) = serde_yaml::from_slice(&r) {
        return env.token;
      }
    }
  }
  None
}
