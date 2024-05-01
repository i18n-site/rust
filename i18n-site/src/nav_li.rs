use std::{collections::HashMap, path::Path};


pub fn nav_li(dir: &Path, lang: &str) -> HashMap<String, String> {
  let fp = dir.join(lang).join("i18n.yml");
  if fp.exists()
    && let Ok(yml) = xerr::ok!(ifs::r(fp))
    && let Ok(m) = xerr::ok!(serde_yaml::from_slice::<HashMap<String, String>>(&yml[..]))
  {
    m
  } else {
    Default::default()
  }
}
