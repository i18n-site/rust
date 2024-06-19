use aok::Result;
use fred::{
  interfaces::HashesInterface,
  prelude::{RedisClient, RedisValue},
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
pub struct UserCache {
  pub global: Box<[u8]>,
  pub user: Box<[u8]>,
  pub kv: RedisClient,
}

impl UserCache {
  pub fn new(kv: RedisClient, prefix: impl AsRef<[u8]>, uid: u64) -> Self {
    let global: Box<[u8]> = concat!(prefix, SPLIT).into();
    let user: Box<[u8]> = concat!(prefix, ">", B255.e(uid), SPLIT).into();
    Self { global, user, kv }
  }
}

impl Cache for UserCache {
  async fn _get(&self, suffix: &[u8], keys: &[&[u8]]) -> Result<Vov> {
    let kv = &self.kv;
    let li: Vov = kv.hmget(concat!(self.user, suffix), keys.to_vec()).await?;
    let (key_li, pm) = PosMerge::new(keys, li);
    let cached: Vov = kv.hmget(concat!(self.global, suffix), key_li).await?;
    Ok(pm.merge(cached))
  }

  async fn _set<'a, V: Send + Into<RedisValue>>(
    &self,
    suffix: &[u8],
    map: Map<'a, V>,
  ) -> Result<()> {
    Ok(self.kv.hset(concat!(self.global, suffix), map).await?)
  }
}
