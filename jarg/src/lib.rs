use axum::{
  body::Bytes,
  extract::{FromRequest, Request},
  http::StatusCode,
  response::{IntoResponse, Response},
};

pub struct Json<T>(pub T);

impl<S, T> FromRequest<S> for Json<T>
where
  Bytes: FromRequest<S>,
  S: Send + Sync,
  T: for<'a> sonic_rs::Deserialize<'a>,
{
  type Rejection = Response;
  async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
    let body = Bytes::from_request(req, state)
      .await
      .map_err(IntoResponse::into_response)?;

    match sonic_rs::from_slice(&body) {
      Ok(r) => Ok(Self(r)),
      Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string()).into_response())?,
    }
  }
}

#[macro_export]
macro_rules! jarg {
  ($($name:ident),*$(,)?) => {
$crate::Json(($($name),*))
  };
}

#[macro_export]
macro_rules! json {
  ($($ty:ty),*$(,)?) => {
$crate::Json::<($($ty),*)>
  };
}
