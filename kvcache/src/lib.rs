#![allow(async_fn_in_trait)]
#![feature(async_closure)]

use std::future::IntoFuture;

use aok::Result;
use fred::{clients::RedisClient, interfaces::HashesInterface};

pub type Map = Vec<(Box<[u8]>, Box<[u8]>)>;
pub type Vov = Vec<Option<Vec<u8>>>;
pub type Vv = Vec<Vec<u8>>;

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

pub trait Cache {
  async fn _get(&self, suffix: &[u8], keys: &[Box<[u8]>]) -> Result<Vov>;
  async fn _set(&self, suffix: &[u8], map: Map) -> Result<()>;

  async fn getset_str<
    K: IntoIterator<Item = Ref> + Send,
    Ref: AsRef<[u8]>,
    F: IntoFuture<Output = Result<Vec<String>>>,
  >(
    &self,
    suffix: impl AsRef<[u8]>,
    keys: K,
    get: impl Fn(Vec<String>) -> F + Copy,
  ) -> Result<Vec<String>> {
    Ok(
      self
        .getset(suffix, keys, async |li| {
          Ok(
            get(
              li.into_iter()
                .map(|i| String::from_utf8_lossy(&i).into())
                .collect(),
            )
            .await?
            .into_iter()
            .map(|i| i.as_bytes().into())
            .collect::<Vec<_>>(),
          )
        })
        .await?
        .into_iter()
        .map(|i| String::from_utf8_lossy(&i).into())
        .collect(),
    )
  }

  async fn getset<
    K: IntoIterator<Item = Ref> + Send,
    Ref: AsRef<[u8]>,
    F: IntoFuture<Output = Result<Vec<Box<[u8]>>>>,
  >(
    &self,
    suffix: impl AsRef<[u8]>,
    keys: K,
    get: impl Fn(Vec<Box<[u8]>>) -> F,
  ) -> Result<Vv> {
    let suffix = suffix.as_ref();
    let mut bin_li: Vec<Box<[u8]>> = Vec::new();
    let hash_li = keys
      .into_iter()
      .map(|i| {
        let i = i.as_ref();
        bin_li.push(i.into());
        xhash::xhash(i)
      })
      .collect::<Vec<_>>();
    let r = self._get(suffix, &hash_li[..]).await?;
    let mut key_li = Vec::new();
    for (pos, i) in r.iter().enumerate() {
      if i.is_none() {
        key_li.push(bin_li[pos].clone());
      }
    }

    if key_li.is_empty() {
      return Ok(r.into_iter().map(|i| i.unwrap()).collect());
    }

    let mut result = Vec::with_capacity(r.len());
    let mut to_set = get(key_li).await?;
    to_set.reverse();
    let mut kv = Vec::with_capacity(to_set.len());
    for (pos, i) in r.into_iter().enumerate() {
      if let Some(i) = i {
        result.push(i);
      } else if let Some(t) = to_set.pop() {
        result.push(t.to_vec());
        kv.push((hash_li[pos].clone(), t));
      } else {
        tracing::error!("get result_li.len != input_li.len");
      }
    }
    self._set(suffix, kv).await?;
    // for (pos, i) in .into_iter().enumerate() {}

    Ok(result)
  }
}
// get: impl Fn(key_li:Vv)->Vv

macro_rules! concat {
  ($($i:expr),*)=>{
    &[
      $(&$i[..]),*
    ].concat()[..]
  }
}

pub struct HsetCache {
  pub hset: Box<[u8]>,
  pub kv: RedisClient,
}

pub const SPLIT: &[u8] = b":";

impl HsetCache {
  pub fn new(kv: RedisClient, prefix: impl AsRef<[u8]>) -> Self {
    Self {
      hset: concat!(prefix.as_ref(), SPLIT).into(),
      kv,
    }
  }
}

impl Cache for HsetCache {
  async fn _set(&self, suffix: &[u8], map: Map) -> Result<()> {
    Ok(self.kv.hmset(concat!(self.hset, suffix), map).await?)
  }

  async fn _get(&self, suffix: &[u8], keys: &[Box<[u8]>]) -> Result<Vov> {
    Ok(
      self
        .kv
        .hmget(concat!(self.hset, suffix), keys.to_vec())
        .await?,
    )
  }
}

pub struct HsetCache2 {
  pub global: Box<[u8]>,
  pub user: Box<[u8]>,
  pub kv: RedisClient,
}

impl HsetCache2 {
  pub fn new(kv: RedisClient, prefix: impl AsRef<[u8]>, id: u64) -> Self {
    let global: Box<[u8]> = concat!(prefix.as_ref(), SPLIT).into();
    let id = intbin::u64_bin(id);
    let user = concat!(global, id, SPLIT).into();
    Self { global, user, kv }
  }
}

impl Cache for HsetCache2 {
  async fn _get(&self, suffix: &[u8], keys: &[Box<[u8]>]) -> Result<Vov> {
    let kv = &self.kv;
    let li: Vov = kv.hmget(concat!(self.user, suffix), keys.to_vec()).await?;
    let (key_li, pm) = PosMerge::new(keys, li);
    let cached: Vov = kv.hmget(concat!(self.global, suffix), key_li).await?;
    Ok(pm.merge(cached))
  }

  async fn _set(&self, suffix: &[u8], map: Map) -> Result<()> {
    Ok(self.kv.hmset(concat!(self.global, suffix), map).await?)
  }
}
