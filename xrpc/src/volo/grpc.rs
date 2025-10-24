use http::Extensions;
use volo_grpc::{Response, Status, metadata::MetadataMap};

use crate::{IntoResponse, Map};

impl Map for &MetadataMap {
  fn get(&self, key: impl AsRef<str>) -> Option<&str> {
    if let Some(v) = MetadataMap::get(self, key.as_ref())
      && let Ok(v) = v.to_str()
    {
      return Some(v);
    }
    None
  }
}

impl<T> crate::Req for volo_grpc::Request<T> {
  fn headers(&self) -> impl Map {
    self.metadata()
  }

  fn extensions(&self) -> &Extensions {
    self.extensions()
  }

  fn extensions_mut(&mut self) -> &mut Extensions {
    self.extensions_mut()
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
