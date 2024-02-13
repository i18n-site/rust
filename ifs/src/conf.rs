use std::{io::Read, path::PathBuf};

use crate::dir::BIN_HOME;

pub trait Msg = prost::Message + std::default::Default;

#[derive(Default)]
pub struct Item<T: Msg> {
  pub dir: Box<str>,
  pub name: String,
  t: std::marker::PhantomData<T>,
}

pub fn log_err(err: std::io::Error) {
  if err.kind() != std::io::ErrorKind::NotFound {
    tracing::error!("{}", err);
  }
}

impl<T: Msg> Item<T> {
  pub fn new(dir: impl AsRef<str>, name: impl Into<String>) -> Self {
    Self {
      dir: Box::from(dir.as_ref()),
      name: name.into(),
      t: std::marker::PhantomData,
    }
  }

  pub fn fp(&self) -> PathBuf {
    BIN_HOME.join(&*self.dir).join("_").join(&self.name)
  }

  pub fn get(&self) -> Option<T> {
    let fp = self.fp();
    match std::fs::File::open(fp) {
      Ok(mut f) => {
        let mut data = Vec::new();
        if f.read_to_end(&mut data).is_ok() {
          if let Ok(r) = T::decode(&data[..]) {
            return Some(r);
          }
        }
      }
      Err(err) => {
        log_err(err);
      }
    }
    None
  }

  pub fn rm(&self) {
    if let Err(err) = std::fs::remove_file(self.fp()) {
      log_err(err);
    }
  }

  pub fn set(&self, msg: T) {
    use std::fs;
    let fp = self.fp();
    xerr::log!(fs::create_dir_all(fp.parent().unwrap()));
    xerr::log!(fs::write(fp, msg.encode_to_vec()));
  }
}

#[macro_export]
macro_rules! conf {
  ($cls:ident { $($name:ident : $ty:ident),* $(,)? }) => {
    pub struct $cls {
      dir: String,
    }
    // $(
    //     pub $name: $crate::conf::Item<api::$ty>
    // ),*

    impl<S: Into<String>> From<S> for $cls {
      fn from(s: S) -> Self {
        Self { dir: s.into() }
      }
    }

    impl $cls {
      $(
        #[allow(non_snake_case)]
        pub fn $name(&self) -> $crate::conf::Item<api::$ty> {
          $crate::conf::Item::new(&self.dir, stringify!($name))
        }
      )*
    }
  }
}
