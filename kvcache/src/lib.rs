#![feature(doc_cfg)]
#![allow(async_fn_in_trait)]

use fred::{
  error::Error,
  prelude::FredResult,
  types::{Map, MultipleKeys},
};

pub type Vov = Vec<Option<Vec<u8>>>;

pub trait Cache {
  fn get_li<K>(&self, hash_li: K) -> impl Future<Output = FredResult<Vov>>
  where
    K: Into<MultipleKeys> + Send,
  {
    self._get_li(hash_li.into())
  }

  fn _get_li(&self, hash_li: MultipleKeys) -> impl Future<Output = FredResult<Vov>>;

  fn set_li<V>(&self, map: V) -> impl Future<Output = FredResult<()>>
  where
    V: TryInto<Map> + Send,
    V::Error: Into<Error> + Send,
  {
    async move {
      match map.try_into() {
        Ok(map) => self._set_li(map).await,
        Err(err) => Err(err.into()),
      }
    }
  }

  fn _set_li(&self, map: Map) -> impl Future<Output = FredResult<()>>;
}

#[cfg(feature = "hset_cache")]
mod hset_cache;

#[cfg(feature = "hset_cache")]
#[doc(cfg(feature = "hset_cache"))]
pub use hset_cache::HsetCache;

#[cfg(feature = "user_cache")]
mod user_cache;

#[cfg(feature = "user_cache")]
#[doc(cfg(feature = "user_cache"))]
pub use user_cache::{UserCache, kvli, prefix_key};
