use std::{
  collections::{HashMap, HashSet},
  sync::atomic::Ordering::Relaxed,
};

use aok::Result;
use len_trait::len::Len;

use crate::{
  api::{Check, HostStateLi, IdName, KindStateLi, State, StateLi},
  cache, cron,
  db::Status,
  RESOLVE_DNS,
};

pub fn kind_state_li(li: impl Len + IntoIterator<Item = Status>) -> Vec<KindStateLi> {
  let mut m: HashMap<u64, HashMap<u64, Vec<State>>> = HashMap::new();
  for i in li {
    let state = State {
      dns_type: i.dns_type as _,
      err: i.err,
      ts: i.ts,
    };
    if let Some(v) = m.get_mut(&i.kind_id) {
      if let Some(h) = v.get_mut(&i.host_id) {
        h.push(state);
      } else {
        v.insert(i.host_id, vec![state]);
      }
    } else {
      m.insert(i.kind_id, HashMap::from([(i.host_id, vec![state])]));
    };
  }
  m.into_iter()
    .map(|i| KindStateLi {
      kind_id: i.0,
      li: i
        .1
        .into_iter()
        .map(|i| HostStateLi {
          host_id: i.0,
          li: i.1,
        })
        .collect(),
    })
    .collect()
}

pub fn to_id_name(li: impl Len + IntoIterator<Item = (u64, String)>) -> Vec<IdName> {
  let mut id_name = Vec::with_capacity(li.len());
  for i in li {
    id_name.push(IdName { id: i.0, name: i.1 });
  }
  id_name
}

pub async fn status() -> Result<StateLi> {
  let li: Vec<Status> = m::q!("SELECT kind_id,host_id,dns_type,err,ts FROM watch");

  let mut host_id_set = HashSet::new();
  let mut ip_id_set = HashSet::new();
  let mut kind_id_set = HashSet::new();

  for i in &li {
    if RESOLVE_DNS.contains(&i.dns_type) {
      host_id_set.insert(i.host_id);
    } else {
      ip_id_set.insert(i.host_id);
    }
    kind_id_set.insert(i.kind_id);
  }

  let mut ok = Vec::with_capacity(li.len());
  let mut err = Vec::new();

  for i in li {
    if i.err == 0 {
      ok.push(i);
    } else {
      err.push(i);
    }
  }

  Ok(StateLi {
    kind: to_id_name(
      cache::kind(kind_id_set)
        .await?
        .into_iter()
        .map(|i| (i.0, i.1.v))
        .collect::<Vec<_>>(),
    ),
    host: to_id_name(cache::host(host_id_set).await?),
    ip: to_id_name(
      cache::ip(ip_id_set)
        .await?
        .into_iter()
        .map(|i| (i.0, i.1.name))
        .collect::<Vec<_>>(),
    ),
    ok: kind_state_li(ok),
    err: kind_state_li(err),
    check: Some(Check {
      last: cron::TS.load(Relaxed),
      count: cron::COUNT.load(Relaxed),
      cost: cron::DURATION.load(Relaxed),
    }),
  })
}
