#![cfg_attr(docsrs, feature(doc_cfg))]

use std::{
  collections::HashMap,
  fmt::{Debug, Display},
  str::FromStr,
};

use tracing::error;

#[cfg(feature = "env")]
mod env;

#[cfg(feature = "env")]
pub use const_str;
#[cfg(feature = "env")]
pub use env::EnvConf;

#[cfg(feature = "fs")]
mod fs;

#[cfg(feature = "fs")]
pub use fs::FsConf;

#[derive(Debug, Clone)]
pub struct Line {
  pub pos: usize,
  pub val: String,
}

#[derive(Debug, Clone, Default)]
pub struct Confer {
  pub li: Vec<String>,
  pub kv: HashMap<String, Line>,
  pub changed: bool,
}

impl Confer {
  pub fn new(txt: impl AsRef<str>) -> Confer {
    let mut li = vec![];
    let mut kv = HashMap::new();
    for i in txt.as_ref().trim().lines() {
      let i = i.trim();
      if i.starts_with("#") {
        continue;
      }
      if let Some((key, val)) = i.split_once(":") {
        let key = key.trim_end();
        let val = val.trim_start();
        kv.insert(
          key.into(),
          Line {
            pos: li.len(),
            val: val.into(),
          },
        );
        li.push(format!("{key}: {val}"));
        continue;
      }
      li.push(i.to_owned());
    }
    Confer {
      li,
      kv,
      changed: false,
    }
  }

  pub fn str(&self, key: impl AsRef<str>) -> Option<&str> {
    let key = key.as_ref();
    if let Some(line) = self.kv.get(key) {
      return Some(&line.val);
    }
    None
  }

  pub fn get<T: FromStr + Display>(&mut self, key: impl AsRef<str>, default: T) -> T {
    let key = key.as_ref();
    if let Some(line) = self.kv.get(key) {
      let val = &line.val;
      match val.parse() {
        Ok(val) => return val,
        Err(_) => {
          error!("config error {}: {} ; use default {}", key, val, default);
        }
      }
    } else {
      self.insert(key, default.to_string());
    }
    default
  }

  fn insert(&mut self, key: &str, val: String) {
    self.changed = true;
    let line = format!("{key}: {val}");
    self.kv.insert(
      key.to_owned(),
      Line {
        pos: self.li.len(),
        val,
      },
    );
    self.li.push(line);
  }

  pub fn set(&mut self, key: impl AsRef<str>, val: impl Display) {
    let key = key.as_ref();
    let val = val.to_string();
    if let Some(line) = self.kv.get_mut(key) {
      if line.val == val {
        return;
      }
      self.changed = true;
      self.li[line.pos] = format!("{key}: {val}");
      line.val = val;
    } else {
      self.insert(key, val);
    }
  }
}

impl Display for Confer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.li.join("\n"))
  }
}
