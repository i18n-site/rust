use aok::{Result, OK};
use map_await::MapAwait;
use rand::{rngs::OsRng, Rng};
use static_init::constructor;
use tokio_stream::StreamExt;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  let mut rng = OsRng;
  let end = rng.gen_range(6..8);
  for range in [0..0, 0..1, 0..4, 0..end] {
    let mut iter = range.clone().map_unordered(3, |i| async move {
      let mut rng = OsRng;
      let sleep = rng.gen_range(1000..2000);
      let i = i + 1;
      info!("{i} begin sleep {}", sleep);
      tokio::time::sleep(std::time::Duration::from_millis(sleep)).await;
      info!("{i} done");
      i
    });
    while let Some(i) = iter.next().await {
      info!("{:?}> {i}", &range);
    }
    info!("------");
  }
  info!("exit");
  OK
}
