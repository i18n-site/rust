use aok::{Result, OK};
use ireq::ReqError;

use crate::{err::errlog, Kind, Watch};

pub async fn curl(
  kind: &Kind,
  watch: Watch,
  host: impl ToString,
  kind_url: impl ToString,
  watch_url: impl ToString,
) -> Result<()> {
  let host = host.to_string();
  let kind_url = kind_url.to_string();
  let watch_url = watch_url.to_string();
  let dns_type = watch.dns_type;
  let url = format!("https://{kind_url}/{}/{host}/{watch_url}", dns_type);

  match ireq::get(&url).await {
    Err(err) => {
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
    }
    Ok(txt) => {
      let mut sql = vec!["UPDATE watch SET "];

      if watch.err != 0 {
        let kind_v = &kind.v;
        let err_duration = crate::err_duration(watch.id).await?;
        let title = format!("✅ {kind_v} {host} ( IPV{dns_type} 恢复正常, 耗时 {err_duration})");
        let txt = "请求响应如下:\n".to_owned() + &txt;
        hi::send(title, txt, url).await;
        sql.push("err=0,");
      }
      let wid = format!(
        "ts={} WHERE id={}",
        sts::sec() + kind.duration as u64,
        watch.id
      );
      sql.push(&wid);
      let sql = xstr::join(sql);
      dbg!(sql);
    }
  }

  OK
}
