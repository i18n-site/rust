#![feature(async_closure)]
#![allow(non_snake_case)]
pub mod api {
  include!(concat!(env!("OUT_DIR"), "/api.rs"));
}
use std::collections::HashSet;

mod cache;
mod ip;
use aok::{Result, OK};
use futures::{stream::FuturesUnordered, StreamExt};
use hook::hook;
use m::q;
use paste::paste;

mod timeout;
use timeout::timeout;
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

pub const RESOLVE_DNS: [u8; 2] = [4, 6];

pub async fn ping() -> Result<()> {
  // let li: Vec<Watch> = q!("SELECT id,host_id,kind_id,dns_type,err,arg_id FROM watch");
  let li: Vec<Watch> = q!(
    "SELECT id,host_id,kind_id,dns_type,err,arg_id FROM watch WHERE ts<=?",
    sts::sec()
  );
  if li.is_empty() {
    return OK;
  }

  let mut kind_set = HashSet::new();
  let mut host_set = HashSet::new();
  let mut arg_set = HashSet::new();
  let mut ip_set = HashSet::new();

  li.iter().for_each(|w| {
    kind_set.insert(w.kind_id);
    if RESOLVE_DNS.contains(&w.dns_type) {
      host_set.insert(w.host_id);
    } else {
      ip_set.insert(w.host_id);
    }
    arg_set.insert(w.arg_id);
  });

  let host_map = cache::host(host_set).await?;
  let ip_map = cache::ip(ip_set).await?;
  let kind_map = cache::kind(kind_set).await?;
  for i in kind_map.values() {
    arg_set.insert(i.arg_id);
  }
  let arg_map = cache::arg(arg_set).await?;

  let mut ing_curl = FuturesUnordered::new();
  let mut ing_hook = FuturesUnordered::new();
  let mut ing_ip = FuturesUnordered::new();

  for watch in &li {
    if RESOLVE_DNS.contains(&watch.dns_type) {
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
                  s
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
    } else if let Some(kind) = kind_map.get(&watch.kind_id) {
      if let Some(ip) = ip_map.get(&watch.host_id) {
        let name = &ip.name;
        ing_ip.push(timeout(kind, name, watch, async move {
          let addr = &ip.ip;
          if let Err(err) = ip::ping(&kind.v, addr).await {
            errlog(kind, name, watch, format!("{addr}\n{err}"), "").await?;
          } else {
            ok(kind, watch, &ip.name, || addr.to_string(), "").await?;
          }
          OK
        }));
      }
    } else {
      dberr!(WatchMissKind "watch_id={} kind_id={}",watch.id, watch.kind_id);
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
  log!(ing_hook, ing_curl, ing_ip);
  OK
}
