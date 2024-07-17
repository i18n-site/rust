use std::path::{Path, PathBuf};

use aok::Result;

use crate::I18nLi;

#[derive(Debug)]
pub struct Htm {
  pub name: String,
  pub htm: String,
  pub has_i18n: bool,
}

pub fn pug(htm: &Path, li: &[PathBuf], i18n_li: &mut I18nLi) -> Result<Vec<Htm>> {
  let mut r = Vec::with_capacity(li.len());
  for name in li {
    let pug = ifs::rtxt(htm.join(name))?;
    let htm = ipug::parse(pug)?.to_str()?;
    let (htm, replaced) = i18n_li.replace(&htm);
    let name = name.display().to_string();

    r.push(Htm {
      name: name[..name.len() - 4].into(),
      htm,
      has_i18n: replaced > 0,
    });
  }
  Ok(r)
}

impl Htm {
  pub fn to_fn(&self) -> String {
    let htm = sonic_rs::to_string(&self.htm).unwrap();
    let htm = &htm[1..htm.len() - 1].replace("\\\"", "\"");
    let func = if self.has_i18n {
      format!("(I)=>`{}`", htm.replace('`', "\\`"))
    } else {
      format!("()=>'{}'", htm.replace('\'', "\\'"))
    };
    format!("{}:{}", self.name, func)
  }
}
