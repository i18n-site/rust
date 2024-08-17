use thiserror::Error;

#[derive(Debug)]
pub struct Time {
  pub year: i32,
  pub month: i8,
  pub day: i8,
  pub hour: i8,
  pub minute: i8,
  pub second: i8,
}

#[derive(Debug, Error)]
pub enum TimeError {
  #[error("Year {0} is out of valid range")]
  InvalidYear(i32),
  #[error("Month {0} is out of valid range")]
  InvalidMonth(i8),
  #[error("Day {0} is out of valid range")]
  InvalidDay(i8),
  #[error("Hour {0} is out of valid range")]
  InvalidHour(i8),
  #[error("Minute {0} is out of valid range")]
  InvalidMinute(i8),
  #[error("Second {0} is out of valid range")]
  InvalidSecond(i8),
}

pub fn is_leap_year(year: u64) -> bool {
  (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn str2ts(date_str: &str) -> Result<u64, TimeError> {
  str2time(date_str).try_into()
}

pub fn str2time(date_str: &str) -> Time {
  let mut year = -1;
  let mut month: i8 = -1;
  let mut day: i8 = -1;
  let mut hour: i8 = -1;
  let mut minute: i8 = -1;
  let mut second: i8 = -1;
  let mut current_number = String::new();

  #[allow(clippy::never_loop)]
  'out: loop {
    for c in date_str.chars() {
      if c.is_ascii_digit() {
        current_number.push(c);
      } else if !current_number.is_empty() {
        if year == -1 {
          year = current_number.parse().unwrap();
        } else {
          let n = current_number.parse().unwrap();
          if month == -1 {
            month = n;
          } else if day == -1 {
            day = n;
          } else if hour == -1 {
            hour = n;
          } else if minute == -1 {
            minute = n;
          } else if second == -1 {
            second = n;
            break 'out;
          }
        }
        current_number.clear();
      }
    }
    if !current_number.is_empty() {
      second = current_number.parse().unwrap();
    }
    break;
  }

  Time {
    year,
    month,
    day,
    hour,
    minute,
    second,
  }
}

impl TryFrom<Time> for u64 {
  type Error = TimeError;

  fn try_from(time: Time) -> Result<Self, Self::Error> {
    let year = if time.year == -1 { 1970 } else { time.year };
    if year < 1970 {
      return Err(TimeError::InvalidYear(time.year));
    }
    let year = year as u64;

    let month = if time.month == -1 { 1 } else { time.month };
    if !(1..=12).contains(&month) {
      return Err(TimeError::InvalidMonth(time.month));
    }
    let month = month as u64;

    let day = if time.day == -1 { 1 } else { time.day };
    let max_days = match month {
      1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
      4 | 6 | 9 | 11 => 30,
      2 if is_leap_year(year) => 29,
      2 => 28,
      _ => unreachable!(), // 已经通过month的检查，这里不可能到达
    };
    if !(1..=max_days).contains(&{ day }) {
      return Err(TimeError::InvalidDay(time.day));
    }
    let day = day as u64;

    let hour = if time.hour == -1 { 0 } else { time.hour };
    if !(0..=23).contains(&hour) {
      return Err(TimeError::InvalidHour(time.hour));
    }
    let hour = hour as u64;

    let minute = if time.minute == -1 { 0 } else { time.minute };
    if !(0..=59).contains(&minute) {
      return Err(TimeError::InvalidMinute(time.minute));
    }
    let minute = minute as u64;

    let second = if time.second == -1 { 0 } else { time.second };
    if !(0..=59).contains(&second) {
      return Err(TimeError::InvalidSecond(time.second));
    }
    let second = second as u64;

    let days_in_month = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

    let mut days =
      (year - 1970) * 365 + (year - 1969) / 4 - (year - 1901) / 100 + (year - 1601) / 400;

    days += days_in_month[month as usize - 1];

    if month > 2 && is_leap_year(year) {
      days += 1;
    }

    days += day - 1;

    Ok(days * 86400 + hour * 3600 + minute * 60 + second)
  }
}
