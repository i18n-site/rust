#![allow(non_snake_case)]

use std::string::ToString;

use aok::{Result, OK};
use futures::{stream::FuturesUnordered, StreamExt};
use hook::hook;
use ireq::ReqError;
use mysql_macro::{exe, mysql_async::prelude::FromRow, q, q01};
use xhash::{HashMap, HashSet};
use xstr::join;

mod db;
use db::{Kind, Watch};
mod watch;
use watch::watch;
mod hook;

macro_rules! dberr {
  ($type:ident $s:expr $(,$t:expr)*) => {{
    let err = format!($s,$($t),*);
    let err_type = stringify!($type);
    let msg = format!("DB ERROR {} : {}",err_type,err);
    tracing::warn!(msg);
    hi::send(err_type,err.clone(),"https://atomgit.com/3ti/rust/blob/main/alive/src/lib.rs#L13").await;
  }};
}

pub async fn id_v(table: &str, id_set: HashSet<u64>) -> Result<HashMap<u64, String>> {
  if id_set.is_empty() {
    return Ok(Default::default());
  }
  let li: Vec<(u64, String)> = q!(format!(
    "SELECT id,v FROM {table} WHERE id IN ({})",
    join(id_set)
  ));
  Ok(HashMap::from_iter(li.into_iter()))
}

pub fn is_power_of_two(n: u32) -> bool {
  (n > 0) && ((n & (n - 1)) == 0)
}

pub fn should_send(err_count: u32, warn_err: u8) -> bool {
  let warn_err = warn_err as _;
  if err_count > warn_err {
    let diff = err_count - warn_err;
    if diff > 86400 {
      diff % 86400 == 0
    } else {
      // 1=4(W) 2=5 3=6 4=7 5=8(W)
      is_power_of_two(diff + 3)
    }
  } else {
    false
  }
}

pub async fn alive(err_count: u32, watch_id: u64) -> Result<String> {
  if err_count > 1 {
    if let Some::<(u8, u64)>((state, ts)) = q01!(format!(
      "SELECT state,ts FROM log WHERE watch_id={watch_id} ORDER BY id DESC LIMIT 1"
    )) {
      if state == 1 {
        let now = sts::sec();
        if now > ts {
          let n = (now - ts) / 60;
          return Ok(format!(", 持续 {n} 分钟"));
        }
      }
    }
  }
  Ok("".to_owned())
}

pub async fn errlog(
  kind: &Kind,
  host: impl AsRef<str>,
  watch: &Watch,
  txt: impl AsRef<str>,
  url: impl AsRef<str>,
) -> Result<()> {
  let host = host.as_ref();
  let txt = txt.as_ref();
  let kind_v = &kind.v;
  let url = url.as_ref();
  let dns_type = watch.dns_type;
  let err_count = watch.err + 1;
  let watch_id = watch.id;

  let mut title = format!("❌ {kind_v} {host} ( IPV{dns_type} 第 {err_count} 次");

  if should_send(err_count, kind.warnErr) {
    let alive = alive(err_count, watch_id).await?;
    title = format!("{title}{alive} )");
    hi::send(&title, txt, url).await;
  } else {
    title += " )";
  }
  tracing::warn!("{title}\n{url}\n{txt}\n",);
  exe!(format!("UPDATE watch SET err=err+1 WHERE id={watch_id}"));
  OK
}

pub async fn curl(
  kind: &Kind,
  watch: Watch,
  host: impl ToString,
  kind_url: impl ToString,
  watch_url: impl ToString,
) {
  let host = host.to_string();
  let kind_url = kind_url.to_string();
  let watch_url = watch_url.to_string();
  let dns_type = watch.dns_type;
  let url = format!("https://{kind_url}/{}/{host}/{watch_url}", dns_type);
  // todo 并发
  if let Err(err) = ireq::get(&url).await {
    let txt = if let Some(ReqError::Status(code, txt)) = err.downcast_ref::<ReqError>() {
      let mut t = code.to_string();
      if !txt.is_empty() {
        t.push('\n');
        t.push_str(txt);
      }
      t
    } else {
      err.to_string()
    };
    xerr::log!(errlog(&kind, host, &watch, txt, url).await);
  } else {
    if watch.err != 0 {
      // 恢复的通知  ✅
    }
    // 更新 watch 的 ts ,加上 kind 的 duration, 设置 err = 0
    todo!();
  }
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
    join(kind_set)
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
          todo!();
          continue;
        }

        if let Some(kind_url) = url_map.get(&kind.url_id) {
          ing.push(curl(&kind, i, host, kind_url, watch_url));
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

  while let Some(()) = ing.next().await {}

  OK
}
