#![allow(non_snake_case)]

use aok::{Result, OK};
use futures::{stream::FuturesUnordered, StreamExt};
use hook::hook;
use mysql_macro::q;
use xhash::{HashMap, HashSet};
use xstr::Join;

mod ok;
use ok::ok;
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
mod err_duration;
mod hook;
use err_duration::err_duration;

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

  let mut ing_curl = FuturesUnordered::new();
  let mut ing_hook = FuturesUnordered::new();

  for pos in 0..li.len() {
    let watch = &li[pos];
    if let Some(host) = host_map.get(&watch.host_id) {
      if let Some(kind) = kind_map.get(&watch.kind_id) {
        let watch_url = if watch.url_id > 0 {
          url_map.get(&watch.url_id).map(|i| i.as_str()).unwrap_or("")
        } else {
          ""
        };
        if let Some(task) = hook(&kind, watch, host) {
          ing_hook.push(task);
        } else {
          if let Some(kind_url) = url_map.get(&kind.url_id) {
            ing_curl.push(curl(kind, watch, host, kind_url, watch_url));
          } else {
            dberr!(
              KindMissUrl
              "watch id={} kind_id={} url_id={}",
              watch.id,
              watch.kind_id,
              watch.url_id
            );
          }
        };
      } else {
        dberr!(WatchMissKind "watch id={} kind_id={}", watch.id, watch.kind_id);
      }
    } else {
      dberr!(WatchMissHost "watch id={} host_id={}", watch.id, watch.host_id);
    }
  }
  macro_rules! log {
    ($($ing:ident),*) => {
      $(
        while let Some(r) = $ing.next().await {
          xerr::log!(r);
        }
      )*
    };
  }
  log!(ing_hook, ing_curl);
  OK
}
