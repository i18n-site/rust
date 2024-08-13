use fred::{
  interfaces::HashesInterface,
  prelude::{RedisClient, RedisResult, RedisValue},
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
  async fn _set_li<'a, V: Into<RedisValue> + Send>(
    &self,
    suffix: &[u8],
    map: Map<'a, V>,
  ) -> RedisResult<()> {
    self.kv.hset(concat!(self.hset, suffix), map).await
  }

  async fn _get_li(&self, suffix: &[u8], keys: &[&[u8]]) -> RedisResult<Vov> {
    self
      .kv
      .hmget(concat!(self.hset, suffix), keys.to_vec())
      .await
  }
}
