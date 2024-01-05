use std::{
  collections::{HashMap, HashSet},
  sync::atomic::Ordering::Relaxed,
};

use aok::Result;
use dashmap::DashMap;
use len_trait::len::Len;
use mysql_macro as m;

use crate::{
  api::{Check, IdName, State, StateLi},
  cron,
  db::Status,
};

#[static_init::dynamic]
pub static HOST: DashMap<u64, String> = DashMap::new();

#[static_init::dynamic]
pub static KIND: DashMap<u64, String> = DashMap::new();

pub fn to_id_name(li: impl Len + IntoIterator<Item = (u64, String)>) -> Vec<IdName> {
  let mut id_name = Vec::with_capacity(li.len());
  for i in li {
    id_name.push(IdName { id: i.0, name: i.1 });
  }
  id_name
}

pub async fn status() -> Result<StateLi> {
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

  Ok(StateLi {
    kind: to_id_name(kind),
    host: to_id_name(host),
    li: li
      .into_iter()
      .map(|i| State {
        kind_id: i.kind_id,
        host_id: i.host_id,
        dns_type: i.dns_type as _,
        err: i.err,
        ts: i.ts,
      })
      .collect(),
    check: Some(Check {
      last: cron::TS.load(Relaxed),
      count: cron::COUNT.load(Relaxed),
      cost: cron::DURATION.load(Relaxed),
    }),
  })
}
