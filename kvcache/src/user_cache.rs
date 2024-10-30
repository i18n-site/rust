use fred::{
  interfaces::KeysInterface,
  prelude::{RedisClient, RedisResult, RedisValue},
};
use rany::{Rany, B255};
use xbin::concat;

use crate::{Cache, Map, Vov};

pub struct PosMerge {
  pub li: Vov,
  pub pos_li: Vec<usize>,
}

pub const SPLIT: &[u8] = b":";

impl PosMerge {
  fn new<T: Clone>(key_li: &[T], li: Vov) -> (Vec<T>, Self) {
    let mut pos_li = Vec::new();
    let mut next_key_li = Vec::new();
    for (pos, i) in li.iter().enumerate() {
      if i.is_none() {
        next_key_li.push(key_li[pos].clone());
        pos_li.push(pos);
      }
    }
    (next_key_li, Self { li, pos_li })
  }

  fn merge(self, li: impl IntoIterator<Item = Option<Vec<u8>>>) -> Vov {
    let mut r = self.li;
    let pos_li = self.pos_li;
    for (pos, i) in li.into_iter().enumerate() {
      if i.is_some() {
        r[pos_li[pos]] = i;
      }
    }
    r
  }
}

#[derive(Clone)]
pub struct UserCache<'a> {
  pub global: Box<[u8]>,
  pub user: Box<[u8]>,
  pub kv: &'a RedisClient,
}

impl<'a> UserCache<'a> {
  pub fn new(kv: &'a RedisClient, prefix: impl AsRef<[u8]>, uid: u64) -> Self {
    let global: Box<[u8]> = concat!(prefix, SPLIT).into();
    let user: Box<[u8]> = concat!(prefix, ">", B255.e(uid), SPLIT).into();
    Self { global, user, kv }
  }

  pub fn kvli<V: Send + Into<RedisValue>>(
    &self,
    prefix: &[u8],
    suffix: &[u8],
    map: Map<'_, V>,
  ) -> Vec<(Box<[u8]>, V)> {
    let prefix = concat!(prefix, suffix, SPLIT);
    let li: Vec<(Box<[u8]>, _)> = map
      .into_iter()
      .map(|(k, v)| (concat!(prefix, k).into(), v))
      .collect();
    li
  }

  pub fn user_kvli<V: Send + Into<RedisValue>>(
    &self,
    suffix: &[u8],
    map: Map<'_, V>,
  ) -> Vec<(Box<[u8]>, V)> {
    self.kvli(&self.user, suffix, map)
  }

  pub fn global_kvli<V: Send + Into<RedisValue>>(
    &self,
    suffix: &[u8],
    map: Map<'_, V>,
  ) -> Vec<(Box<[u8]>, V)> {
    self.kvli(&self.global, suffix, map)
  }
}

pub fn prefix_key(prefix: &[u8], suffix: &[u8], keys: &[&[u8]]) -> Vec<Box<[u8]>> {
  let prefix = concat!(prefix, suffix, SPLIT);
  keys.iter().map(|k| concat!(prefix, k).into()).collect()
}

impl<'a> Cache for UserCache<'a> {
  async fn _get_li(&self, suffix: &[u8], keys: &[&[u8]]) -> RedisResult<Vov> {
    let kv = &self.kv;
    let li: Vov = kv.mget(prefix_key(&self.user, suffix, keys)).await?;
    let (key_li, pm) = PosMerge::new(keys, li);
    let cached: Vov = kv.mget(prefix_key(&self.global, suffix, &key_li)).await?;
    Ok(pm.merge(cached))
  }

  async fn _set_li<'b, V: Send + Into<RedisValue>>(
    &self,
    suffix: &[u8],
    map: Map<'b, V>,
  ) -> RedisResult<()> {
    self.kv.mset(self.global_kvli(suffix, map)).await
  }
}
