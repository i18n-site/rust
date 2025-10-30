use chrono::{DateTime, Utc};
use coarsetime::Clock;

pub fn cookie_expire_with_days(days: u64) -> String {
  let now = Clock::now_since_epoch();
  let future_time = now + coarsetime::Duration::from_secs(days * 24 * 60 * 60);
  let datetime = DateTime::<Utc>::from_timestamp(future_time.as_secs() as i64, 0).unwrap();
  datetime.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}

pub fn cookie_expire() -> String {
  // 从 Chrome M104（2022 年 8 月）开始，Cookie 所设置的有效期不能超过 400 天。 https://developer.chrome.com/blog/cookie-max-age-expires
  cookie_expire_with_days(400)
}
