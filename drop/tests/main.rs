use aok::{OK, Void};
use drop::leak;
use tracing::info;

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
  {
    leak!(t = Test(1));
    let ing = tokio::spawn(run(t));
    ing.await?;
  }
  {
    let t = Test(1);
    leak!(t);
    let ing = tokio::spawn(run(t));
    ing.await?;
  }
  OK
}
