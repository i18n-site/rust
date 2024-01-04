use std::sync::atomic::Ordering::Relaxed;

pub async fn ping() -> aerr::msg!() {
  let pre = alive::cron::TS.load(Relaxed);

  Ok("todo".to_owned())
}
