use serde::{Deserialize, Serialize};

use crate::{conf, I18nLi, EMPTY};

#[derive(Debug, Serialize, Deserialize)]
pub struct Nav {
  pub i18n: String,
  pub url: String,
  pub r#use: String,
  pub menu: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavLi(pub Vec<Nav>);

impl NavLi {
  pub fn json(&self) -> Result<String, sonic_rs::Error> {
    let mut r = Vec::with_capacity(self.0.len());
    for i in &self.0 {
      r.push(format!("{}>{}>{}", i.url, i.r#use, i.menu));
    }
    sonic_rs::to_string(&r.join(";"))
  }

  pub fn i18n_li(&self) -> I18nLi {
    I18nLi(self.0.iter().map(|nav| nav.i18n.clone()).collect())
  }

  pub fn new(nav_li: &[conf::Nav]) -> Self {
    Self(
      nav_li
        .iter()
        .map(|nav| {
          let menu = nav.menu.clone().unwrap_or(EMPTY);
          let r#use = nav.r#use.clone();
          let i18n = nav.i18n.clone();
          let url = nav.url.clone().unwrap_or(i18n.clone());
          Nav {
            i18n,
            url,
            r#use,
            menu,
          }
        })
        .collect(),
    )
  }
}
