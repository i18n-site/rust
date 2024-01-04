use std::{
  sync::atomic::{AtomicU64, Ordering},
  time::{Instant, SystemTime},
};

use tokio::time::{interval, Duration};

pub static START_TIME: AtomicU64 = AtomicU64::new(0);
pub static RUNNING_DURATION: AtomicU64 = AtomicU64::new(0);
pub static RUNNING_COUNT: AtomicU64 = AtomicU64::new(0);

async fn run_your_function() {
  // 这里是你的函数实现
  // 例如我们让它等待1秒
  tokio::time::sleep(Duration::from_secs(1)).await;
}

async fn _run() {
  let now = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_secs();
  START_TIME.store(now, Ordering::Relaxed);

  let start = Instant::now();
  run_your_function().await;
  let function_running_duration = start.elapsed().as_secs();

  RUNNING_DURATION.fetch_add(function_running_duration, Ordering::Relaxed);
  RUNNING_COUNT.fetch_add(1, Ordering::Relaxed);
}

pub async fn run() {
  let mut interval = interval(Duration::from_secs(60));

  loop {
    interval.tick().await;
    _run().await;
  }
}
