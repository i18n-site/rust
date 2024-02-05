use aok::{Result, OK};
use ireq::ReqError;

use crate::{errlog, ok, Kind, Watch};

pub async fn curl<'a>(
  kind: &'a Kind,
  watch: &'a Watch,
  host: &'a str,
  kind_arg: &'a str,
  watch_arg: &'a str,
) -> Result<()> {
  let dns_type = watch.dns_type;
  let url = format!("https://{kind_arg}/{}/{host}/{watch_arg}", dns_type);

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
      xerr::log!(errlog(kind, host, watch, txt, url).await);
    }
    Ok(txt) => {
      ok(
        kind,
        watch,
        host,
        || "请求响应如下:\n".to_owned() + &txt,
        url,
      )
      .await?;
    }
  }

  OK
}
