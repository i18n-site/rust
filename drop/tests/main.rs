use aok::{OK, Void};
use tracing::info;
use drop::leak;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[derive(Debug)]
pub struct Test(pub i32);

pub async fn run(t: &Test) {
  info!("> obj {:?}", t);
}

#[tokio::test]
async fn test() -> Void {
  // 不要写 let t = leak(Test(1)).ptr; 这样会导致过早释放
  let t = leak(Test(1));

  let ing = tokio::spawn(run(t.ptr));

  ing.await?;

  OK
}
