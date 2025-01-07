use std::ops::Range;

pub use coarsetime::{self, Clock, Duration};
use num_traits::cast::AsPrimitive;

pub const DAY: i64 = 86400;

pub fn readable(n: u64) -> String {
  if n < 100 {
    format!("{}s", n)
  } else if n < 100 * 60 {
    format!("{}m", n / 60)
  } else if n < 100 * 60 * 60 {
    format!("{}h", n / 3600)
  } else {
    format!("{}d", n / 86400)
  }
}

pub fn now() -> Duration {
  Clock::now_since_epoch()
}

pub fn sec() -> u64 {
  now().as_secs()
}

pub fn ms() -> u64 {
  now().as_millis()
}

pub fn nano() -> u64 {
  now().as_nanos()
}

pub fn min() -> u64 {
  now().as_mins()
}

pub fn hour() -> u64 {
  now().as_hours()
}

pub fn day() -> u64 {
  now().as_days()
}

pub fn day_month(day: impl AsPrimitive<i32>) -> i32 {
  sec_month((day.as_() as i64) * DAY)
}

pub fn sec_month(sec: impl AsPrimitive<i64>) -> i32 {
  use chrono::{DateTime, Datelike, Utc};

  let datetime: DateTime<Utc> = DateTime::from_timestamp(sec.as_(), 0).unwrap();
  let year = datetime.year();
  let month = datetime.month() as i32;
  ym_n(year, month)
}

pub fn ym_n(year: i32, month: i32) -> i32 {
  (year - 1970) * 12 + month
}

pub fn n_ym(n: i32) -> (i32, i32) {
  let n = n - 1;
  let year = 1970 + n / 12;
  let month = 1 + n % 12;
  (year, month)
}

pub fn now_month() -> i32 {
  sec_month(sec())
}

/// 前闭后开
pub fn month_sec(month: i32) -> Range<i64> {
  use chrono::{TimeZone, Utc};
  let month = month - 1;
  let year = 1970 + month / 12;
  let month = (month % 12 + 1) as u32;

  let this_month_start = Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap();
  let next_month_start = if month == 12 {
    Utc.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0)
  } else {
    Utc.with_ymd_and_hms(year, month + 1, 1, 0, 0, 0)
  }
  .unwrap();

  let this_month_start_sec = this_month_start.timestamp();
  let this_month_end_sec = next_month_start.timestamp();

  this_month_start_sec..this_month_end_sec
}

pub fn month_day(month: i32) -> Range<i32> {
  let r = month_sec(month);
  ((r.start / DAY) as i32)..((r.end / DAY) as i32)
}

pub fn now_month_day() -> Range<i32> {
  use chrono::{DateTime, Datelike, TimeZone, Utc};
  let now: DateTime<Utc> = DateTime::from_timestamp(sec() as _, 0).unwrap();
  let begin = now.with_day(1).unwrap();
  let begin = begin.timestamp() / 86400;
  let year = now.year();
  let month = now.month();
  let next_month_start = if month == 12 {
    Utc.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0)
  } else {
    Utc.with_ymd_and_hms(year, month + 1, 1, 0, 0, 0)
  }
  .unwrap();

  let end = next_month_start.timestamp() / 86400;
  (begin as i32)..(end as i32)
}
