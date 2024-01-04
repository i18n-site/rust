use std::sync::atomic::Ordering::Relaxed;

use tokio::time::{sleep, Duration};

pub async fn ping() -> aerr::msg!() {
  let pre = alive::cron::TS.load(Relaxed);
  let now = sts::sec();
  let diff = if now > pre { now - pre } else { 0 };

  if diff > 300 {
    tracing::error!("alive cron expire");
    tokio::spawn(async {
      sleep(Duration::from_secs(3)).await;
      std::process::exit(1);
    });
  }

  Ok(diff.to_string())
}
