use std::{
  collections::{HashMap, HashSet},
  sync::atomic::Ordering::Relaxed,
};

use aok::Result;
use dashmap::DashMap;
use mysql_macro as m;

use crate::{cron, db::Status};

#[derive(Debug)]
pub struct StatusLi(
  HashMap<u64, String>,          // host
  HashMap<u64, String>,          // kind
  Vec<(u64, u64, u8, u32, u64)>, // li
  u64,                           // pre start
  u64,                           // run times
  u64,                           // run secs
);

#[static_init::dynamic]
pub static HOST: DashMap<u64, String> = DashMap::new();

#[static_init::dynamic]
pub static KIND: DashMap<u64, String> = DashMap::new();

pub async fn status() -> Result<StatusLi> {
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
    for i in m::id_v_str("host", host_id_li).await? {
      HOST.insert(i.0, i.1);
    }
  }
  if !kind_id_li.is_empty() {
    for i in m::id_v_str("kind", kind_id_li).await? {
      KIND.insert(i.0, i.1);
    }
  }

  let mut kind = HashMap::new();
  let mut host = HashMap::new();

  for i in &li {
    if let Some(v) = KIND.get(&i.kind_id).map(|i| i.clone()) {
      kind.insert(i.kind_id, v);
    }
    if let Some(v) = HOST.get(&i.host_id).map(|i| i.clone()) {
      host.insert(i.host_id, v);
    }
  }

  Ok(StatusLi(
    kind,
    host,
    li.into_iter()
      .map(|i| (i.kind_id, i.host_id, i.dns_type, i.err, i.ts))
      .collect(),
    cron::TS.load(Relaxed),
    cron::COUNT.load(Relaxed),
    cron::DURATION.load(Relaxed),
  ))
}
