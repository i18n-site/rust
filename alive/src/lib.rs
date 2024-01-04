#![allow(non_snake_case)]

use aok::{Result, OK};
use futures::{stream::FuturesUnordered, StreamExt};
use hook::hook;
use mysql_macro::{q, q01};
use xhash::{HashMap, HashSet};
use xstr::Join;

mod recover;
use recover::recover;
mod curl;
use curl::curl;
mod db;
use db::{Kind, Watch};
mod watch;
use watch::watch;
mod should_send;
use should_send::should_send;
mod err;
use err::errlog;
mod hook;

pub async fn id_v(table: &str, id_set: HashSet<u64>) -> Result<HashMap<u64, String>> {
  if id_set.is_empty() {
    return Ok(Default::default());
  }
  let li: Vec<(u64, String)> = q!(format!(
    "SELECT id,v FROM {table} WHERE id IN ({})",
    id_set.join(",")
  ));
  Ok(HashMap::from_iter(li.into_iter()))
}

pub async fn err_duration(watch_id: u64) -> Result<String> {
  if let Some::<(u8, u64)>((state, ts)) = q01!(format!(
    "SELECT state,ts FROM log WHERE watch_id={watch_id} ORDER BY id DESC LIMIT 1"
  )) {
    if state == 1 {
      let now = sts::sec();
      if now > ts {
        let n = (now - ts) / 60;
        return Ok(format!("{n} 分钟"));
      }
    }
  }
  Ok("".to_owned())
}

pub async fn next() -> Result<()> {
  let now = sts::sec();

  let li: Vec<Watch> = q!(
    "SELECT id,host_id,kind_id,dns_type,err,url_id FROM watch WHERE ts<=?",
    now
  );
  if li.is_empty() {
    return OK;
  }

  let mut kind_set = HashSet::default();
  let mut host_set = HashSet::default();
  let mut url_set = HashSet::default();

  li.iter().for_each(|w| {
    kind_set.insert(w.kind_id);
    host_set.insert(w.host_id);
    if w.url_id != 0 {
      url_set.insert(w.url_id);
    }
  });

  let kind_li: Vec<Kind> = q!(format!(
    "SELECT id,url_id,duration,warnErr,v FROM kind WHERE id IN ({})",
    kind_set.join(",")
  ));

  kind_li.iter().for_each(|k| {
    if k.url_id != 0 {
      url_set.insert(k.url_id);
    }
  });

  let kind_map = HashMap::<u64, Kind>::from_iter(kind_li.into_iter().map(|k| (k.id, k)));
  let host_map = id_v("host", host_set).await?;
  let url_map = id_v("url", url_set).await?;

  let mut ing = FuturesUnordered::new();

  for i in li {
    if let Some(host) = host_map.get(&i.host_id) {
      if let Some(kind) = kind_map.get(&i.kind_id) {
        let watch_url = if i.url_id > 0 {
          url_map.get(&i.url_id).map(|i| i.as_str()).unwrap_or("")
        } else {
          ""
        };

        if hook(&kind.v).await {
          continue;
        }

        if let Some(kind_url) = url_map.get(&kind.url_id) {
          ing.push(curl(kind, i, host, kind_url, watch_url));
        } else {
          dberr!(
            KindMissUrl
            "watch id={} kind_id={} url_id={}",
            i.id,
            i.kind_id,
            i.url_id
          );
        }
      } else {
        dberr!(WatchMissKind "watch id={} kind_id={}", i.id, i.kind_id);
      }
    } else {
      dberr!(WatchMissHost "watch id={} host_id={}", i.id, i.host_id);
    }
  }

  while ing.next().await.is_some() {}

  OK
}
