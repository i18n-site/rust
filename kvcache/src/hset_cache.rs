use fred::{
  interfaces::HashesInterface,
  prelude::{Client, FredResult},
  types::{Map, MultipleKeys},
};

use crate::{Cache, Vov};

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
  async fn _set_li(&self, map: Map) -> FredResult<()> {
    self.kv.hset(&self.hset[..], map).await
  }

  async fn _get_li(&self, keys: MultipleKeys) -> FredResult<Vov> {
    self.kv.hmget(&self.hset[..], keys).await
  }
}
