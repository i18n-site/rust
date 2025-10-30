use fred::{
  interfaces::KeysInterface,
  prelude::{Client, FredResult},
  types::{Map, MultipleKeys, Value, Value::Bytes},
};
use xbin::concat;

use crate::{Cache, Vov};

pub struct PosMerge {
  pub li: Vov,
  pub pos_li: Vec<usize>,
}

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
  pub kv: &'a Client,
}

pub fn kvli(prefix: &[u8], map: Map) -> Vec<(Box<[u8]>, Value)> {
  map
    .inner()
    .into_iter()
    .map(|(k, v)| (concat!(prefix, k.as_bytes()).into(), v))
    .collect()
}

pub fn prefix_key<B: AsRef<[u8]>>(
  prefix: &[u8],
  keys: impl IntoIterator<Item = B>,
) -> Vec<Box<[u8]>> {
  keys
    .into_iter()
    .map(|k| concat!(prefix, k.as_ref()).into())
    .collect()
}

pub const SPLIT: &[u8] = b":";

pub fn user_prefix(prefix: impl AsRef<[u8]>, kind: impl AsRef<[u8]>, user_id: u64) -> Box<[u8]> {
  let prefix = prefix.as_ref();
  let kind = b255::encode(kind.as_ref());
  concat!(prefix, b255::encode(vb::e([user_id])), SPLIT, kind, SPLIT).into()
}

impl<'a> UserCache<'a> {
  pub fn new(
    kv: &'a Client,
    prefix: impl AsRef<[u8]>,
    kind: impl AsRef<[u8]>,
    user_id: u64,
  ) -> Self {
    let kind = b255::encode(kind.as_ref());
    let prefix = prefix.as_ref();
    let global: Box<[u8]> = concat!(prefix, kind, SPLIT).into();

    let user: Box<[u8]> =
      concat!(prefix, b255::encode(vb::e([user_id])), SPLIT, kind, SPLIT).into();
    Self { global, user, kv }
  }

  pub fn global_kvli(&self, map: Map) -> Vec<(Box<[u8]>, Value)> {
    kvli(&self.global, map)
  }
}

impl Cache for UserCache<'_> {
  async fn _get_li(&self, key_li: MultipleKeys) -> FredResult<Vov> {
    use fred::prelude::Value::String;
    let kv = &self.kv;
    let key_li = key_li.into_values();
    let mut keys = Vec::with_capacity(key_li.len());
    for i in key_li.iter() {
      match i {
        String(i) => keys.push(i.as_bytes()),
        Bytes(i) => keys.push(i),
        _ => {}
      }
    }

    let li: Vov = kv.mget(prefix_key(&self.user, &keys)).await?;
    let (key_li, pm) = PosMerge::new(&keys, li);
    let cached: Vov = kv.mget(prefix_key(&self.global, &key_li)).await?;
    Ok(pm.merge(cached))
  }

  async fn _set_li(&self, map: Map) -> FredResult<()> {
    self.kv.mset(self.global_kvli(map)).await
  }
}
