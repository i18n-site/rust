use std::time::Duration;

use aok::Result;
use axum::{Router, routing::get};
use axum_graceful_restart::serve;

#[tokio::main]
async fn main() -> Result<()> {
  loginit::init();
  let app = Router::new().route("/", get(handler));

  serve("0.0.0.0:8899".parse()?, app).await
}

async fn handler() -> String {
  let pid = std::process::id();
  println!("new conn");
  tokio::time::sleep(Duration::from_secs(10)).await;
  format!("PID: {pid}")
}
