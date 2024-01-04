use aok::{Result, OK};
use mysql_macro::{exe, mysql_async::prelude::FromRow, q, q01};

use crate::{
  db::{Kind, Watch},
  should_send, 故障持续时间,
};

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
    let alive = 故障持续时间(err_count, watch_id).await?;
    title = format!("{title}{alive} )");
    hi::send(&title, txt, url).await;
  } else {
    title += " )";
  }
  tracing::warn!("{title}\n{url}\n{txt}\n",);
  exe!(format!("UPDATE watch SET err=err+1 WHERE id={watch_id}"));
  OK
}

#[macro_export]
macro_rules! dberr {
  ($type:ident $s:expr $(,$t:expr)*) => {{
    let err = format!($s,$($t),*);
    let err_type = stringify!($type);
    let msg = format!("DB ERROR {} : {}",err_type,err);
    tracing::warn!(msg);
    hi::send(err_type,err.clone(),"https://atomgit.com/3ti/rust/blob/main/alive/src/lib.rs#L13").await;
  }};
}
