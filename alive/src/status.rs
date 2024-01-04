use std::collections::{HashMap, HashSet};

use aok::{Result, OK};
use dashmap::DashMap;
use mysql_macro as m;
use sonic_rs::{Deserialize, Serialize};

use crate::{db::Status, id_v};

#[derive(Serialize, Deserialize)]
pub struct StatusLi {
  host: HashMap<u64, String>,
  kind: HashMap<u64, String>,
  li: Vec<Status>,
}

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

  let mut kind_map = HashMap::new();
  let mut host_map = HashMap::new();

  for i in li {
    if let Some(v) = KIND.get(&i.kind_id).map(|i| i.clone()) {
      kind_map.insert(i.kind_id, v);
    }
    if let Some(v) = HOST.get(&i.host_id).map(|i| i.clone()) {
      host_map.insert(i.host_id, v);
    }
  }

  OK
}
