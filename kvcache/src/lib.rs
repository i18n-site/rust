#![allow(async_fn_in_trait)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use fred::{
  error::Error,
  prelude::FredResult,
  types::{Map, MultipleKeys},
};

pub type Vov = Vec<Option<Vec<u8>>>;

pub trait Cache {
  async fn get_li<K>(&self, hash_li: K) -> FredResult<Vov>
  where
    K: Into<MultipleKeys> + Send,
  {
    let hash_li = hash_li.into();
    if hash_li.len() == 0 {
      return Ok(Vec::new());
    }
    self._get_li(hash_li).await
  }

  fn _get_li(&self, hash_li: MultipleKeys) -> impl Future<Output = FredResult<Vov>>;

  async fn set_li<V>(&self, map: V) -> FredResult<()>
  where
    V: TryInto<Map> + Send,
    V::Error: Into<Error> + Send,
  {
    match map.try_into() {
      Ok(map) => {
        if map.is_empty() {
          Ok(())
        } else {
          self._set_li(map).await
        }
      }
      Err(err) => Err(err.into()),
    }
  }

  fn _set_li(&self, map: Map) -> impl Future<Output = FredResult<()>>;
}

#[cfg(feature = "hset_cache")]
mod hset_cache;

#[cfg(feature = "hset_cache")]
pub use hset_cache::HsetCache;

#[cfg(feature = "user_cache")]
mod user_cache;

#[cfg(feature = "user_cache")]
pub use user_cache::{UserCache, kvli, prefix_key, user_prefix};
