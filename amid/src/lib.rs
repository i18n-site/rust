use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};

pub fn middleware(r: anyhow::Result<Response>) -> Response {
  match r {
    Ok(r) => r,
    Err(err) => {
      tracing::error!("{:?}", err);
      (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
    }
  }
}
