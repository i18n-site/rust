use std::{
  future::Future,
  sync::atomic::{AtomicU64, Ordering},
  time::Instant,
};

use tokio::time::{interval, Duration};

pub static TS: AtomicU64 = AtomicU64::new(0);
pub static DURATION: AtomicU64 = AtomicU64::new(0);
pub static COUNT: AtomicU64 = AtomicU64::new(0);

async fn _run() {
  let now = sts::sec();
  TS.store(now, Ordering::Relaxed);

  let start = Instant::now();
  xerr::log!(crate::ping().await);
  let duration = start.elapsed().as_millis() as u64;
  DURATION.fetch_add(duration, Ordering::Relaxed);
  COUNT.fetch_add(1, Ordering::Relaxed);
}

pub async fn run<Fut>(next: impl Fn() -> Fut)
where
  Fut: Future<Output = ()> + Send + 'static,
{
  let mut interval = interval(Duration::from_secs(60));

  loop {
    interval.tick().await;
    _run().await;
    tokio::task::spawn(next());
  }
}
