/*

#[cfg(feature = "macro")]
mod test_macro {
}
*/

#[tokio::test]
async fn test() -> aok::Result<()> {
  loginit::init();
  use std::collections::HashSet;

  use m::{id_v_str, q};

  let id_set = HashSet::from([11]);
  // dbg!(id_set.join(","));

  let map = id_v_str("arg", id_set).await?;
  dbg!(map);

  let r: Vec<(u64, String)> = q!("select id,v from arg where id IN(11)");
  dbg!(r);
  // let li = alive::status().await?;
  // use alive::Alive;
  //
  // let mut alive = Alive::new();
  // alive.ping().await?;
  // alive.ping().await?;
  Ok(())
}
