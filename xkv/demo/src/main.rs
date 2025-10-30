use aok::{OK, Result};
use xkv::{R, fred::interfaces::KeysInterface, log::info};

async fn test_redis() -> Result<()> {
  let key = "xkvtest1";
  let val = "abc";
  R!(del key);

  let v: bool = R.exists(key).await?;
  assert!(!v);

  let v: Option<String> = R.get(key).await?;
  info!("get {key} = {:?}", v);
  assert_eq!(v, None);

  R!(set key, val, None, None, false);

  let v: Option<String> = R.get(key).await?;
  info!("get {key} = {:?}", v);
  assert_eq!(v, Some(val.into()));

  R!(del key);

  OK
}

#[tokio::main]
async fn main() -> Result<()> {
  // 仅在程序启动的main函数中调用一次 / Call it only once in the main function of the program
  static_::init().await?;
  test_redis().await?;
  OK
}
