use std::fmt;

pub use anyhow;
use axum::body::{Body, Bytes};
pub use axum::{
  extract::FromRequestParts,
  http::{StatusCode, request::Parts},
  response::{IntoResponse, Response},
};

#[derive(Debug, Clone)]
pub struct ErrorResponse(StatusCode, Bytes);

impl std::error::Error for ErrorResponse {}

impl fmt::Display for ErrorResponse {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self.0)
  }
}

pub fn err<T>(code: StatusCode, msg: impl Into<Bytes>) -> anyhow::Result<T, ErrorResponse> {
  Err(ErrorResponse(code, msg.into()))
}

impl IntoResponse for ErrorResponse {
  fn into_response(self) -> Response<Body> {
    (self.0, self.1).into_response()
  }
}

#[macro_export]
macro_rules! from_request_parts {
  ($cls:ty, $func:ident) => {
    impl<S> $crate::FromRequestParts<S> for $cls
    where
      S: Send + Sync,
    {
      type Rejection = $crate::Response;

      async fn from_request_parts(
        parts: &mut $crate::Parts,
        _state: &S,
      ) -> Result<Self, Self::Rejection> {
        use std::sync::Arc;

        use $crate::{IntoResponse, anyhow};

        let r: anyhow::Result<Self> = $func(parts).await;

        match r {
          Ok(r) => Ok(r),
          Err(error) => Err(
            if let Some(r) = error.downcast_ref::<$crate::ErrorResponse>() {
              r.clone().into_response()
            } else {
              ($crate::StatusCode::BAD_REQUEST, error.to_string()).into_response()
            },
          ),
        }
      }
    }
  };
}
