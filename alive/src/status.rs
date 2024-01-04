use std::collections::HashSet;

use aok::{Result, OK};
use dashmap::DashMap;
use mysql_macro as m;

use crate::{db::Status, id_v};

#[static_init::dynamic]
pub static HOST: DashMap<u64, String> = DashMap::new();

#[static_init::dynamic]
pub static KIND: DashMap<u64, String> = DashMap::new();

pub async fn status() -> Result<()> {
  let li: Vec<Status> =
    m::q!("SELECT kind_id,host_id,dns_type,err,ts FROM watch ORDER BY err DESC,kind_id,host_id");

  let mut host_id_li = HashSet::new();
  let mut kind_id_li = HashSet::new();

  for i in &li {
    if !HOST.contains_key(&i.host_id) {
      host_id_li.insert(i.host_id);
    }
    if !KIND.contains_key(&i.kind_id) {
      kind_id_li.insert(i.kind_id);
    }
  }

  if !host_id_li.is_empty() {
    for i in id_v("host", host_id_li).await? {
      HOST.insert(i.0, i.1);
    }
  }
  if !kind_id_li.is_empty() {
    for i in id_v("kind", kind_id_li).await? {
      KIND.insert(i.0, i.1);
    }
  }

  for s in li {
    tracing::info!(
      "{} {} ipv{} {} {}",
      KIND.get(&s.kind_id).map(|i| i.clone()).unwrap_or_default(),
      HOST.get(&s.host_id).map(|i| i.clone()).unwrap_or_default(),
      s.dns_type,
      s.err,
      s.ts
    );
  }

  OK
}
