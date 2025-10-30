use std::cell::RefCell;

use http::Extensions;
use volo_grpc::{Request, Response, Status, metadata::MetadataMap};

use crate::{ExtVal, HeadersExt, Init, IntoResponse, Map, ParseError};

impl Map for MetadataMap {
  fn get(&self, key: impl AsRef<str>) -> Option<&str> {
    if let Some(v) = MetadataMap::get(self, key.as_ref())
      && let Ok(v) = v.to_str()
    {
      return Some(v);
    }
    None
  }
}

pub fn split<T>(
  req: Request<T>,
) -> (
  HeadersExt<MetadataMap, RefCell<Extensions>>,
  std::result::Result<T, ParseError>,
) {
  let (metadata, extensions, args) = req.into_parts();
  (
    HeadersExt {
      headers: metadata,
      ext: extensions.into(),
    },
    Ok(args),
  )
}

impl crate::Ext for RefCell<Extensions> {
  async fn ext<T: ExtVal + Init>(&self, headers: &impl Map) -> anyhow::Result<T> {
    Ok(if let Some(v) = self.borrow().get::<T>() {
      v.clone()
    } else {
      let v = T::init(headers).await?;
      let mut ext = self.borrow_mut();
      ext.insert(v.clone());
      v
    })
  }
}

pub type Result<T> = std::result::Result<Response<T>, Status>;

impl<T, R: Into<crate::Result<T>>> IntoResponse<Result<T>> for R {
  fn into_response(self) -> Result<T> {
    use crate::Result;
    match self.into() {
      Result::Ok(v) => Ok(Response::new(v)),
      Result::Response(r) => Err(Status::new(
        Into::into(r.code as i32),
        String::from_utf8_lossy(&r.body),
      )),
      Result::Err(err) => Err(Status::internal(err.to_string())),
    }
  }
}
