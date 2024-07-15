use aok::Result;
use fred::{
  interfaces::HashesInterface,
  prelude::{RedisClient, RedisValue},
};
use xbin::concat;

use crate::{Cache, Map, Vov};

#[derive(Clone)]
pub struct HsetCache {
  pub hset: Box<[u8]>,
  pub kv: RedisClient,
}

impl HsetCache {
  pub fn new(kv: RedisClient, prefix: impl AsRef<[u8]>) -> Self {
    Self {
      hset: prefix.as_ref().into(),
      kv,
    }
  }
}

impl Cache for HsetCache {
  async fn _set<'a, V: Into<RedisValue> + Send>(
    &self,
    suffix: &[u8],
    map: Map<'a, V>,
  ) -> Result<()> {
    Ok(self.kv.hset(concat!(self.hset, suffix), map).await?)
  }

  async fn _get(&self, suffix: &[u8], keys: &[&[u8]]) -> Result<Vov> {
    Ok(
      self
        .kv
        .hmget(concat!(self.hset, suffix), keys.to_vec())
        .await?,
    )
  }
}
