use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use tracing::error;

#[derive(Debug)]
pub enum Error {
  Any(anyhow::Error),
  Response(Response),
}

#[derive(Debug)]
pub struct Err(pub Error);

pub type Result<T, E = Err> = anyhow::Result<T, E>;

// Tell axum how to convert `Err` into a response.
impl IntoResponse for Err {
  fn into_response(self) -> Response {
    let err = self.0;
    match err {
      Error::Any(err) => if err.is::<crate::form::Error>() {
        (StatusCode::EXPECTATION_FAILED, err.to_string())
      } else if err.is::<crate::bad_request::Error>() {
        (StatusCode::BAD_REQUEST, err.to_string())
      } else {
        error!("{}\n{}", err.backtrace(), err);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("ERR: {}", err))
      }
      .into_response(),
      Error::Response(r) => r,
    }
  }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into `Result<_, Err>`. That way you don't need to do that manually.
impl<E> From<E> for Err
where
  E: Into<anyhow::Error>,
{
  default fn from(err: E) -> Self {
    Self(Error::Any(err.into()))
  }
}

pub fn err<T>(code: StatusCode, body: impl Into<crate::Msg>) -> Result<T, Err> {
  let mut res = body.into().into_response();
  *res.status_mut() = code;
  std::result::Result::Err(Err(Error::Response(res)))?
}
