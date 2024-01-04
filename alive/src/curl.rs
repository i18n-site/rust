use aok::{Result, OK};
use ireq::ReqError;
use mysql_macro::exe;

use crate::{err::errlog, recover, Kind, Watch};

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
      xerr::log!(errlog(kind, host, &watch, txt, url).await);
    }
    Ok(txt) => {
      let txt = "请求响应如下:\n".to_owned() + &txt;
      recover(kind, watch, host, txt, url).await?;
    }
  }

  OK
}
