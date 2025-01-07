use fred::{
  interfaces::HashesInterface,
  prelude::{Client, FredResult as FredResult, Value},
};
use xbin::concat;

use crate::{Cache, Map, Vov};

#[derive(Clone)]
pub struct HsetCache {
  pub hset: Box<[u8]>,
  pub kv: Client,
}

impl HsetCache {
  pub fn new(kv: Client, prefix: impl AsRef<[u8]>) -> Self {
    Self {
      hset: prefix.as_ref().into(),
      kv,
    }
  }
}

impl Cache for HsetCache {
  async fn _set_li<'a, V: Into<Value> + Send>(
    &self,
    suffix: &[u8],
    map: Map<'a, V>,
  ) -> FredResult<()> {
    self.kv.hset(concat!(self.hset, suffix), map).await
  }

  async fn _get_li(&self, suffix: &[u8], keys: &[&[u8]]) -> FredResult<Vov> {
    self
      .kv
      .hmget(concat!(self.hset, suffix), keys.to_vec())
      .await
  }
}
