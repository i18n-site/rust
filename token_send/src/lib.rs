use std::collections::HashMap;

use aerr::Result;
use api_token::API_TOKEN;
use axum::{Router, extract::Path, http::StatusCode, response::Response, routing::post};
use s_::EMPTY_REF;
use sonic_rs::from_slice;

pub async fn _send(body: &str) -> Result<()> {
  let data: HashMap<String, String> = from_slice(body.as_bytes())?;
  let title = data.get("title").unwrap_or(EMPTY_REF);
  let txt = data.get("txt").unwrap_or(EMPTY_REF);
  let url = data.get("url").unwrap_or(EMPTY_REF);
  hi::send(title, txt, url).await;
  Ok(())
}

#[axum::debug_handler]
pub async fn send(Path(token): Path<String>, body: String) -> Result<Response> {
  if token != *API_TOKEN {
    return aerr::err(StatusCode::UNAUTHORIZED, "error token");
  }
  _send(&body).await?;
  aerr::none()
}

pub fn route(router: Router) -> Router {
  router.route("/send/{token}", post(send))
}
