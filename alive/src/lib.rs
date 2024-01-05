#![allow(non_snake_case)]

use std::collections::{HashMap, HashSet};

use aok::{Result, OK};
use futures::{stream::FuturesUnordered, StreamExt};
use hashlru::Cache;
use hook::hook;
use mysql_macro::{id_v, id_v_str, q};
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

pub struct Alive {
  arg_cache: Cache<u64, String>,
  kind_cache: Cache<u64, Kind>,
}

impl Alive {
  pub fn new() -> Self {
    Alive {
      arg_cache: Cache::new(1024),
      kind_cache: Cache::new(1024),
    }
  }
  pub async fn ping(&mut self) -> Result<()> {
    // let li: Vec<Watch> = q!( "SELECT id,host_id,kind_id,dns_type,err,arg_id FROM watch" );
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
    let mut arg_map = HashMap::new();

    macro_rules! arg_set {
      ($w:ident) => {
        if $w.arg_id > 0 {
          if let Some(exist) = self.arg_cache.get(&$w.arg_id) {
            arg_map.insert($w.arg_id, exist.to_owned());
          } else {
            arg_set.insert($w.arg_id);
          }
        }
      };
    }

    li.iter().for_each(|w| {
      kind_set.insert(w.kind_id);
      host_set.insert(w.host_id);
      arg_set!(w);
    });

    let kind_li: Vec<Kind> = q!(format!(
      "SELECT id,arg_id,duration,warnErr,v FROM kind WHERE id IN ({})",
      kind_set.join(",")
    ));

    kind_li.iter().for_each(|k| {
      arg_set!(k);
    });

    let kind_map = HashMap::<u64, Kind>::from_iter(kind_li.into_iter().map(|k| (k.id, k)));
    let host_map = id_v_str("host", host_set).await?;

    dbg!(arg_set.len());
    if !arg_set.is_empty() {
      for i in id_v_str("arg", arg_set).await? {
        arg_map.insert(i.0, i.1.to_owned());
        self.arg_cache.insert(i.0, i.1);
      }
    }

    dbg!(&arg_map);
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
}
