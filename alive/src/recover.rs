use aok::{Result, OK};
use mysql_macro::exe;

use crate::{Kind, Watch};

pub async fn recover(
  kind: &Kind,
  watch: Watch,
  host: impl AsRef<str>,
  txt: impl AsRef<str>,
  url: impl AsRef<str>,
) -> Result<()> {
  let host = host.as_ref();
  let txt = txt.as_ref();
  let url = url.as_ref();
  let dns_type = watch.dns_type;

  let mut sql = vec!["UPDATE watch SET "];

  if watch.err != 0 {
    let kind_v = &kind.v;
    let err_duration = crate::err_duration(watch.id).await?;
    let title = format!("✅ {kind_v} {host} ( IPV{dns_type} 恢复正常, 耗时 {err_duration})");
    hi::send(title, txt, url).await;
    sql.push("err=0,");
  }
  let wid = format!(
    "ts={} WHERE id={}",
    sts::sec() + kind.duration as u64,
    watch.id
  );
  sql.push(&wid);
  exe!(sql.join(""));
  OK
}
