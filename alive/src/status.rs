use std::collections::HashSet;

use aok::{Result, OK};
use dashmap::DashMap;
use mysql_macro::q;

use crate::db::Status;

#[static_init::dynamic]
pub static HOST: DashMap<u64, String> = DashMap::new();

#[static_init::dynamic]
pub static KIND: DashMap<u64, String> = DashMap::new();

pub async fn status() -> Result<()> {
  let li: Vec<Status> =
    q!("SELECT kind_id,host_id,dns_type,err,ts FROM watch ORDER BY err DESC,kind_id,host_id");

  let mut host_id_li = HashSet::new();
  let mut kind_id_li = HashSet::new();

  for i in li {
    if !HOST.contains_key(&i.host_id) {
      host_id_li.insert(i.host_id);
    }
    if !KIND.contains_key(&i.kind_id) {
      kind_id_li.insert(i.kind_id);
    }
  }

  // if kind_id_li.is_empty() && host_id_li.is_empty() {
  //   return OK;
  // }

  OK
}
