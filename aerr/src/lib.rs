use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use tracing::error;

#[derive(Debug)]
pub enum Err {
  Any(anyhow::Error),
  Response(Box<Response>),
}

#[derive(Debug)]
pub struct Error(pub Err);

pub type Result<T, E = Error> = anyhow::Result<T, E>;

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    let err = self.0;
    match err {
      Err::Any(err) => {
        error!("{}\n{}", err.backtrace(), err);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("ERR: {err}")).into_response()
      }
      Err::Response(r) => *r,
    }
  }
}

impl<E> From<E> for Error
where
  E: Into<anyhow::Error>,
{
  fn from(err: E) -> Self {
    Self(Err::Any(err.into()))
  }
}

pub fn none() -> Result<Response> {
  Ok((StatusCode::NO_CONTENT, b"").into_response())
}

pub fn ok(body: impl IntoResponse) -> Result<impl IntoResponse> {
  Ok(body.into_response())
}

pub fn err<T>(code: StatusCode, body: impl IntoResponse) -> Result<T, Error> {
  let mut res = body.into_response();
  *res.status_mut() = code;
  std::result::Result::Err(Error(Err::Response(Box::new(res))))?
}
