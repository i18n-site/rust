use aok::{Result, OK};

#[tokio::test]
async fn conn() -> Result<()> {
  // let redis = xkv::conn("R").await?;
  // let key = "xkv-test";
  // redis.del(key).await?;
  // assert_eq!(redis.get::<Option<String>, _>(key).await?, None);
  // let val = "值 abc";
  // redis.set(key, val, None, None, false).await?;
  // let get_val = redis.get::<Option<String>, _>(key).await?;
  // assert_eq!(get_val, Some(val.into()));
  // redis.del(key).await?;
  // assert_eq!(redis.get::<Option<String>, _>(key).await?, None);
  OK
}
