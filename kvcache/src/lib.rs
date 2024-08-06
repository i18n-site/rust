#![feature(doc_cfg)]
#![allow(async_fn_in_trait)]
#![feature(async_closure)]

use aok::Result;
use fred::prelude::RedisValue;

pub type Map<'a, V> = Vec<(&'a [u8], V)>;
pub type Vov = Vec<Option<Vec<u8>>>;
// pub type Vv = Vec<Box<[u8]>>;

// pub type VecBin = Vec<Box<[u8]>>;

// pub trait GetBinLi {
//   async fn get<T: AsRef<[u8]>>(&self, bin_li: &[T], state: State<'_>) -> Result<Vec<Box<[u8]>>>;
// }
//
// pub trait GetStrLi {
//   async fn get(&self, bin_li: &[&str], state: State<'_>) -> Result<Vec<String>>;
// }
//
// impl<S: GetStrLi> GetBinLi for S {
//   async fn get<T: AsRef<[u8]>>(&self, bin_li: &[T], state: State<'_>) -> Result<Vec<Box<[u8]>>> {
//     let bin_li = bin_li
//       .as_ref()
//       .iter()
//       .map(|i| unsafe { std::str::from_utf8_unchecked(i.as_ref()) })
//       .collect::<Vec<_>>();
//
//     Ok(
//       self
//         .get(&bin_li, state)
//         .await?
//         .into_iter()
//         .map(|i| i.as_bytes().into())
//         .collect(),
//     )
//   }
// }
//
// pub struct State<'a> {
//   pub no_cache_pos: &'a [usize],
// }

pub trait Cache {
  async fn get_li(&self, suffix: &[u8], bin_li: &[&[u8]]) -> Result<Vov>;
  async fn set_li<'a, V: Send + Into<RedisValue>>(
    &self,
    suffix: &[u8],
    map: Map<'a, V>,
  ) -> Result<()>;

  // async fn getset_str(
  //   &self,
  //   suffix: impl AsRef<[u8]>,
  //   str_li: &[&str],
  //   hash_li: &[&[u8]],
  //   g: impl GetBinLi,
  // ) -> Result<Vec<String>> {
  //   let bin_li = str_li.iter().map(|i| i.as_bytes()).collect::<Vec<_>>();
  //   let r = self.getset(suffix, &bin_li[..], hash_li, g).await?;
  //   Ok(
  //     r.into_iter()
  //       .map(|i| String::from_utf8_lossy(&i).into())
  //       .collect(),
  //   )
  // }
  //
  // async fn getset(
  //   &self,
  //   suffix: impl AsRef<[u8]>,
  //   bin_li: &[&[u8]],
  //   hash_li: &[&[u8]],
  //   g: impl GetBinLi,
  // ) -> Result<Vv> {
  //   let len = bin_li.len();
  //   if len == 0 {
  //     return Ok(Default::default());
  //   }
  //
  //   let suffix = suffix.as_ref();
  //
  //   let r = self._get(suffix, hash_li).await?;
  //   let mut no_cache_pos = Vec::with_capacity(len);
  //   let mut result: Vv = Vec::with_capacity(len);
  //
  //   let no_cache_key: Vec<_> = r
  //     .into_iter()
  //     .zip(bin_li)
  //     .enumerate()
  //     .filter_map(|(pos, (i, key))| match i {
  //       Some(i) => {
  //         result.push(i.into());
  //         None
  //       }
  //       None => {
  //         no_cache_pos.push(pos);
  //         result.push(Default::default());
  //         Some(key)
  //       }
  //     })
  //     .collect();
  //
  //   if !no_cache_key.is_empty() {
  //     let kv: Vec<_> = g
  //       .get(
  //         &no_cache_key,
  //         State {
  //           no_cache_pos: &no_cache_pos,
  //         },
  //       )
  //       .await?
  //       .into_iter()
  //       .zip(no_cache_pos)
  //       .map(|(v, pos)| {
  //         result[pos].clone_from(&v);
  //         (hash_li[pos], v)
  //       })
  //       .collect();
  //     self._set(suffix, kv).await?;
  //     //     tracing::error!("get result_li.len != input_li.len");
  //   };

  // Ok(result)
  // }
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
pub use user_cache::UserCache;
