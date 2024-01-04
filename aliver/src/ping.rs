use std::sync::atomic::Ordering::Relaxed;

pub async fn ping() -> aerr::msg!() {
  let pre = alive::cron::TS.load(Relaxed);
  let now = sts::sec();
  let diff = if now > pre { now - pre } else { 0 };

  if diff > 300 {
    std::process::exit(1);
  }

  Ok(diff.to_string())
}
