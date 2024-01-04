#![allow(non_snake_case)]
#![feature(integer_atomics)]

use std::collections::{HashMap, HashSet};

use aok::{Result, OK};
use futures::{stream::FuturesUnordered, StreamExt};
use hook::hook;
use mysql_macro::q;
use paste::paste;
use xstr::Join;

pub mod cron;
mod ok;
use ok::ok;
mod status;
pub use status::status;
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
mod id_v;
use id_v::id_v;

pub async fn ping() -> Result<()> {
  let now = sts::sec();

  let li: Vec<Watch> = q!(
    "SELECT id,host_id,kind_id,dns_type,err,arg_id FROM watch WHERE ts<=?",
    now
  );
  if li.is_empty() {
    return OK;
  }

  let mut kind_set = HashSet::new();
  let mut host_set = HashSet::new();
  let mut arg_set = HashSet::new();

  li.iter().for_each(|w| {
    kind_set.insert(w.kind_id);
    host_set.insert(w.host_id);
    if w.arg_id != 0 {
      arg_set.insert(w.arg_id);
    }
  });

  let kind_li: Vec<Kind> = q!(format!(
    "SELECT id,arg_id,duration,warnErr,v FROM kind WHERE id IN ({})",
    kind_set.join(",")
  ));

  kind_li.iter().for_each(|k| {
    if k.arg_id != 0 {
      arg_set.insert(k.arg_id);
    }
  });

  let kind_map = HashMap::<u64, Kind>::from_iter(kind_li.into_iter().map(|k| (k.id, k)));
  let host_map = id_v("host", host_set).await?;
  let arg_map = id_v("arg", arg_set).await?;

  let mut ing_curl = FuturesUnordered::new();
  let mut ing_hook = FuturesUnordered::new();

  for watch in &li {
    if let Some(host) = host_map.get(&watch.host_id) {
      if let Some(kind) = kind_map.get(&watch.kind_id) {
        tracing::info!(
          "{} {} IPV{} ERR {}",
          kind.v,
          host,
          watch.dns_type,
          watch.err
        );
        macro_rules! arg {
          ($type:ident) => {
            if $type.arg_id > 0 {
              if let Some(s) = arg_map.get(&$type.arg_id) {
                s.as_str()
              } else {
                paste! {
                  dberr!(
                    [< $type MissArg >]
                    "{} watch_id={} arg_id={} kind_id={} kind_arg_id={}",
                    host,
                    watch.id,
                    watch.arg_id,
                    watch.kind_id,
                    kind.arg_id
                  );
                }
                continue;
              }
            } else {
              ""
            }
          };
        }

        let kind_arg = arg!(kind);
        let watch_arg = arg!(watch);

        if let Some(task) = hook(kind, watch, host, kind_arg, watch_arg) {
          ing_hook.push(task);
        } else {
          ing_curl.push(curl(kind, watch, host, kind_arg, watch_arg));
        }
      } else {
        dberr!(WatchMissKind "{} watch_id={} kind_id={}",host, watch.id, watch.kind_id);
      }
    } else {
      dberr!(WatchMissHost "watch_id={} host_id={}", watch.id, watch.host_id);
    }
  }
  macro_rules! log {
    ($($ing:ident),*) => {
      $(
        while let Some(r) = $ing.next().await {
          if let Err(err) = r {
            let title = "出错了";
            tracing::error!("{title}:\n{err}");
            hi::send(title,err.to_string(),"https://atomgit.com/3ti/rust/blob/main/alive/src/lib.rs#L117").await;
          }
        }
      )*
    };
  }
  log!(ing_hook, ing_curl);
  OK
}
