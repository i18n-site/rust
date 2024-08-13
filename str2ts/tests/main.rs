use std::time::{SystemTime, UNIX_EPOCH};

use aok::{Result, OK};
use chrono::{TimeZone, Utc};
use static_init::constructor;
use str2ts::str2ts;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let sys_ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
  if let Some(date_time) = Utc.timestamp_opt(sys_ts as _, 0).single() {
    let date_str = date_time.format("%Y-%m-%d %H:%M:%S").to_string();
    let ts = str2ts(&date_str)?;
    info!("ts: {}", ts);
    assert_eq!(sys_ts, ts);
  }
  OK
}
