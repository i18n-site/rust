use aok::{Result, OK};

use crate::{Kind, Watch};

pub async fn ok(
  kind: &Kind,
  watch: &Watch,
  host: impl AsRef<str>,
  txt_gen: impl FnOnce() -> String, // txt
  url: impl AsRef<str>,
) -> Result<()> {
  let mut sql = vec!["UPDATE watch SET "];
  if watch.err > 0 {
    sql.push("err=0,");
    if watch.err > kind.warnErr as u32 {
      let host = host.as_ref();
      let url = url.as_ref();
      let txt = txt_gen();
      let dns_type = watch.dns_type;
      let kind_v = &kind.v;
      let err_duration = crate::err_duration(watch.id).await?;
      let title = format!("{kind_v} ✅ {host} ( IPV{dns_type} 恢复正常, 耗时 {err_duration})");
      hi::send(title, txt, url).await;
    }
  }
  let wid = format!(
    "ts={} WHERE id={}",
    sts::sec() + kind.duration as u64,
    watch.id
  );
  sql.push(&wid);
  m::e!(sql.join(""));
  OK
}
