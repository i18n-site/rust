use std::{
  any::{Any, TypeId},
  sync::Arc,
};

use dashmap::{DashMap, mapref::entry::Entry};
use http::HeaderMap;
use pilota::{Bytes, pb::Message};

use crate::{Ext, ExtVal, Init, Map, ParseError};

impl Map for HeaderMap {
  fn get(&self, key: impl AsRef<str>) -> Option<&str> {
    if let Some(v) = HeaderMap::get(self, key.as_ref())
      && let Ok(v) = v.to_str()
    {
      return Some(v);
    }
    None
  }
}

pub type ExtMap = DashMap<TypeId, Box<dyn Any + Send + Sync>>;
// pub struct Req {
//   pub parts: Parts,
//   pub ext: ,
// }

impl Ext for ExtMap {
  async fn ext<T: ExtVal + Init>(&self, headers: &impl Map) -> anyhow::Result<T> {
    match self.entry(TypeId::of::<T>()) {
      Entry::Occupied(o) => {
        // another thread got the lock and inserted before us
        // we'll use its value and drop ours
        // 另一个线程在我们之前获取了锁并插入了值
        // 我们将使用它的值并丢弃我们的值
        Ok(o.get().downcast_ref::<T>().cloned().unwrap())
      }
      Entry::Vacant(map) => {
        let v = T::init(headers).await?;
        // we got the lock and the value is still not there
        // insert our initialized value
        // 我们获取了锁并且值仍然不存在
        // 插入我们初始化的值
        map.insert(Box::new(v.clone()));
        Ok(v)
      }
    }
  }
}

pub type Req = Arc<crate::HeadersExt<HeaderMap, ExtMap>>;

pub type ParseResult<T> = std::result::Result<T, ParseError>;

pub fn parse<T: Message + Default>(bytes: impl Into<Bytes>) -> ParseResult<T> {
  // let (parts, body) = req.into_parts();
  // let body: Body = body.into();
  // let body: Bytes = body.into_bytes().await.unwrap_or_default();
  // (
  //   HeadersExt {
  //     headers: parts.headers,
  //     ext: Default::default(),
  //   },
  let bytes = bytes.into();
  match T::decode(bytes.clone()) {
    Ok(r) => Ok(r),
    Err(e) => Err(ParseError {
      bytes,
      err: e.into(),
    }),
  }
  // )
}
