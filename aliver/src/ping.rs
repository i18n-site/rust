use std::sync::atomic::Ordering::Relaxed;

use axum::http::StatusCode;
use re::err;
use tokio::time::{sleep, Duration};

pub async fn ping() -> re::msg!() {
  let pre = alive::cron::TS.load(Relaxed);
  let now = sts::sec();
  let diff = if now > pre { now - pre } else { 0 };

  if diff > 300 {
    tracing::error!("alive cron expire");
    tokio::spawn(async {
      sleep(Duration::from_secs(3)).await;
      std::process::exit(1);
    });
    err(
      StatusCode::FAILED_DEPENDENCY,
      "alive cron expire".to_owned(),
    )?
  }

  Ok(diff.to_string())
}
