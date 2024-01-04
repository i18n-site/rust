use std::net::SocketAddr;

use axum::{body::Bytes, http::StatusCode, routing::post, Router};
use dotenv::dotenv;
mod mail;

genv::s!(WARN_URL);
genv::def!(PORT:u16 | 8080);

async fn index(json: Bytes) -> aerr::msg!() {
  let subject;
  let txt;
  let status;
  match sonic_rs::from_slice::<mail::Root>(&json) {
    Err(e) => {
      txt = String::from_utf8_lossy(&json);
      let e = e.to_string();
      subject = format!("mailhook json parse error : {}", &e);
      tracing::error!("{}\n{}", e, &txt);
      status = StatusCode::BAD_REQUEST;
    }
    Ok(root) => {
      status = StatusCode::OK;
      let payload = root.payload;
      subject = payload.subject;
      txt = payload.txt.into();
    }
  }
  wxpush::send(&*WARN_URL, &subject, &txt).await?;
  if status == StatusCode::OK {
    Ok(())
  } else {
    aerr::err(status, ())
  }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().ok();

  loginit::init();
  let app = Router::new().route("/", post(aerr::FnAny(index)));
  let addr = SocketAddr::from(([0, 0, 0, 0], PORT()));

  tracing::info!("http://{}", addr);

  let listener = tokio::net::TcpListener::bind(addr).await?;
  axum::serve(listener, app).await?;

  Ok(())
}
