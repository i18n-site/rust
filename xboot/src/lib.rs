use aok::Result;
use static_init::constructor;

#[constructor(0)]
extern "C" fn cinit() {
  loginit::init();
}

use linkme::distributed_slice;

pub type Task = tokio::task::JoinHandle<Result<()>>;

pub type AsyncFn = fn() -> Task;

#[distributed_slice]
pub static ASYNC: [AsyncFn];

pub async fn init() -> Result<()> {
  for i in ASYNC {
    i().await??;
  }
  Ok(())
}
