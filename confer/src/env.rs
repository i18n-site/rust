use crate::Confer;

pub struct EnvConf {
  pub conf: Confer,
  pub prefix: String,
}

impl EnvConf {
  pub fn new(conf: Confer, prefix: impl Into<String>) -> Self {
    Self {
      conf,
      prefix: prefix.into(),
    }
  }

  pub fn str(&self, key: impl AsRef<str>) -> Option<String> {
    let key = key.as_ref();
    if let Ok(val) = std::env::var(self.prefix.clone() + &key.to_uppercase()) {
      return Some(val);
    }
    if let Some(val) = self.conf.str(key) {
      return Some(val.into());
    }
    None
  }
}

#[macro_export]
macro_rules! env_conf {
  ($path: expr) => {{
    $crate::EnvConf::new(
      $crate::FsConf::new($path).load()?,
      $crate::const_str::concat!(
        $crate::const_str::convert_ascii_case!(upper, env!("CARGO_PKG_NAME")),
        "_"
      ),
    )
  }};
}

// pub fn str(&self, key: impl AsRef<str>) -> Option<&str> {
//   let key = key.as_ref();
//   if let Some(line) = self.kv.get(key) {
//     return Some(&line.val);
//   }
//   None
// }
