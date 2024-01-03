#![allow(non_snake_case)]

use aok::{Result, OK};
use hook::hook;
use ireq::ReqError;
use mysql_macro::{mysql_async::prelude::FromRow, q};
use xhash::{HashMap, HashSet};
use xstr::join;

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

#[derive(Debug, Clone, FromRow)]
pub struct Kind {
  pub id: u64,
  pub url_id: u64,
  pub duration: u32,
  pub warnErr: u8,
  pub v: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct Watch {
  pub id: u64,
  pub host_id: u64,
  pub kind_id: u64,
  pub dns_type: u8,
  pub err: u32,
  pub url_id: u64,
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

pub async fn errlog(
  kind: impl AsRef<str>,
  host: impl AsRef<str>,
  watch: &Watch,
  txt: impl AsRef<str>,
  url: impl AsRef<str>,
) {
  let kind = kind.as_ref();
  let host = host.as_ref();
  let txt = txt.as_ref();
  let url = url.as_ref();

  let err_count = watch.err + 1;
  let alive = if err_count > 1 {
    todo!();
    format!("持续 5 分钟")
  } else {
    "".to_owned()
  };
  let dns_type = watch.dns_type;
  let title = format!("❌ 第 {err_count} 次{alive} : {kind} {host} IPV{dns_type}");
  dbg!((title, txt, url));
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

  for i in li {
    if let Some(host) = host_map.get(&i.host_id) {
      if let Some(kind) = kind_map.get(&i.kind_id) {
        let watch_url = if i.url_id > 0 {
          url_map.get(&i.url_id).map(|i| i.as_str()).unwrap_or("")
        } else {
          ""
        };

        let kind_v = &kind.v;
        if hook(kind_v).await {
          todo!();
          continue;
        }

        let dns_type = i.dns_type;

        if let Some(kind_url) = url_map.get(&kind.url_id) {
          let url = format!("https://{kind_url}/{}/{host}/{watch_url}", dns_type);
          // todo 并发
          if let Err(err) = ireq::get(&url).await {
            let txt = if let Some(ReqError::Status(code, url, txt)) = err.downcast_ref::<ReqError>()
            {
              format!("{code}\n{txt}")
            } else {
              err.to_string()
            };
            errlog(&kind_v, host, &i, txt, url).await;
          }
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
  OK
}
