use std::collections::{HashMap, HashSet};

use aok::Result;
use mysql_async::prelude::FromRow;
use mysql_macro::q;
use xstr::Join;

pub async fn id_v<S: Send + 'static>(table: &str, id_set: HashSet<u64>) -> Result<HashMap<u64, S>>
where
  (u64, S): mysql_async::prelude::FromRow,
{
  if id_set.is_empty() {
    return Ok(Default::default());
  }
  let li: Vec<(u64, S)> = q!(format!(
    "SELECT id,v FROM {table} WHERE id IN ({})",
    id_set.join(",")
  ));
  Ok(HashMap::from_iter(li.into_iter()))
}
