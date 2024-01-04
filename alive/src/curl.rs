use aok::{Result, OK};
use ireq::ReqError;

use crate::{errlog, ok, Kind, Watch};

pub async fn curl(
  kind: &Kind,
  watch: &Watch,
  host: impl ToString,
  kind_arg: impl ToString,
  watch_arg: impl ToString,
) -> Result<()> {
  let host = host.to_string();
  let kind_arg = kind_arg.to_string();
  let watch_arg = watch_arg.to_string();
  let dns_type = watch.dns_type;
  let arg = format!("https://{kind_arg}/{}/{host}/{watch_arg}", dns_type);

  match ireq::get(&arg).await {
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
      xerr::log!(errlog(kind, host, &watch, txt, arg).await);
    }
    Ok(txt) => {
      ok(
        kind,
        &watch,
        host,
        || "请求响应如下:\n".to_owned() + &txt,
        arg,
      )
      .await?;
    }
  }

  OK
}
