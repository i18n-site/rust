use aok::{Result, OK};

use crate::{
  db::{Kind, Watch},
  err_duration, should_send,
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

  let mut title = format!("{kind_v} ❌ {host} ( IPV{dns_type} 第 {err_count} 次");

  if should_send(err_count, kind.warnErr) {
    let alive = if err_count > 1 {
      let msg = err_duration(watch_id).await?;
      if msg.is_empty() {
        format!(", 持续 {msg}")
      } else {
        "".into()
      }
    } else {
      "".into()
    };
    title = format!("{title}{alive} )");
    hi::send(&title, txt, url).await;
  } else {
    title += " )";
  }
  tracing::warn!("{title}\n{url}\n{txt}\n",);
  m::e!(format!("UPDATE watch SET err=err+1 WHERE id={watch_id}"));
  OK
}

#[macro_export]
macro_rules! dberr {
  ($type:ident $s:expr $(,$t:expr)*) => {{
    let err = format!($s,$($t),*);
    let err_type = stringify!($type);
    let msg = format!("DB ERROR {} : {}",err_type,err);
    tracing::warn!(msg);
    hi::send(err_type,err,"https://atomgit.com/3ti/rust/blob/main/alive/src/lib.rs#L13").await;
  }};
}
