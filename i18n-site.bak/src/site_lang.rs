use std::{collections::HashMap, path::Path};

use crate::{api, EMPTY};

pub fn nav_i18n(dir: &Path, lang: &str) -> HashMap<String, String> {
  if let Ok(yml) = xerr::ok!(ifs::r(dir.join(lang).join("i18n.yml")))
    && let Ok(m) = xerr::ok!(serde_yaml::from_slice::<HashMap<String, String>>(&yml[..]))
  {
    m
  } else {
    Default::default()
  }
}

pub fn site_lang(
  nav_li: &[String],
  dir: &Path,
  lang: &str,
  url_v_li: Vec<String>,
) -> api::SiteLang {
  api::SiteLang {
    nav_i18n_li: nav_li
      .iter()
      .map(|nav| nav_i18n(dir, lang).get(nav).unwrap_or(&EMPTY).to_owned())
      .collect(),
    url_v_li,
  }
}
