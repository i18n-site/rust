use aok::Result;
use m::q01;

pub async fn err_duration(watch_id: u64) -> Result<String> {
  if let Some::<(u8, u64)>((state, ts)) = q01!(format!(
    "SELECT state,ts FROM log WHERE watch_id={watch_id} ORDER BY id DESC LIMIT 1"
  )) {
    if state == 1 {
      let now = sts::sec();
      if now > ts {
        let n = (now - ts) / 60;
        return Ok(format!("{n} 分钟"));
      }
    }
  }
  Ok("".to_owned())
}
