use ireq::ReqError;

use crate::{err::errlog, Kind, Watch};

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
