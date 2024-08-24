pub const SECONDS_PER_MINUTE: u64 = 60;
pub const SECONDS_PER_DAY: u64 = 86400;
pub const DAYS_IN_MONTHS: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

pub fn is_leap_year(year: u64) -> bool {
  (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub const fn leap_years(year: u64) -> u64 {
  (year / 4) - (year / 100) + (year / 400)
}

pub const LEAP_YEARS_1969: u64 = leap_years(1969);

pub const fn leap_years_since_1970(year: u64) -> u64 {
  leap_years(year) - LEAP_YEARS_1969
}

fn calculate_month_and_day(days: u64, is_leap: bool) -> (u64, u64) {
  let days_in_february = if is_leap { 29 } else { 28 };
  let month;
  let day;

  if days < 31 {
    month = 1;
    day = days + 1;
  } else if days < 31 + days_in_february {
    month = 2;
    day = days - 31 + 1;
  } else if days < 31 + days_in_february + 31 {
    month = 3;
    day = days - 31 - days_in_february + 1;
  } else if days < 31 + days_in_february + 31 + 30 {
    month = 4;
    day = days - 31 - days_in_february - 31 + 1;
  } else if days < 31 + days_in_february + 31 + 30 + 31 {
    month = 5;
    day = days - 31 - days_in_february - 31 - 30 + 1;
  } else if days < 31 + days_in_february + 31 + 30 + 31 + 30 {
    month = 6;
    day = days - 31 - days_in_february - 31 - 30 - 31 + 1;
  } else if days < 31 + days_in_february + 31 + 30 + 31 + 30 + 31 {
    month = 7;
    day = days - 31 - days_in_february - 31 - 30 - 31 - 30 + 1;
  } else if days < 31 + days_in_february + 31 + 30 + 31 + 30 + 31 + 31 {
    month = 8;
    day = days - 31 - days_in_february - 31 - 30 - 31 - 30 - 31 + 1;
  } else if days < 31 + days_in_february + 31 + 30 + 31 + 30 + 31 + 31 + 30 {
    month = 9;
    day = days - 31 - days_in_february - 31 - 30 - 31 - 30 - 31 - 31 + 1;
  } else if days < 31 + days_in_february + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 {
    month = 10;
    day = days - 31 - days_in_february - 31 - 30 - 31 - 30 - 31 - 31 - 30 + 1;
  } else if days < 31 + days_in_february + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30 {
    month = 11;
    day = days - 31 - days_in_february - 31 - 30 - 31 - 30 - 31 - 31 - 30 - 31 + 1;
  } else {
    month = 12;
    day = days - 31 - days_in_february - 31 - 30 - 31 - 30 - 31 - 31 - 30 - 31 - 30 + 1;
  }

  (month, day)
}

pub fn ymdhms(mut sec: u64) -> (u64, u64, u64, u64, u64, u64) {
  let seconds = sec % SECONDS_PER_MINUTE;
  sec /= SECONDS_PER_MINUTE;
  let minutes = sec % SECONDS_PER_MINUTE;
  sec /= SECONDS_PER_MINUTE;
  let hours = sec % 24;
  sec /= 24;

  let mut year = 1970;
  let mut days = sec;

  // Estimate the year
  let mut year_increment = days / 365;
  let temp_year = year + year_increment;

  let total_leap_years_until_temp_year = leap_years_since_1970(temp_year);
  let total_days_until_temp_year = 365 * year_increment + total_leap_years_until_temp_year;

  let diff = if days < total_days_until_temp_year {
    year_increment -= 1;
    365 * year_increment + leap_years_since_1970(year + year_increment)
  } else {
    total_days_until_temp_year
  };

  year += year_increment;
  days -= diff;

  let is_leap = is_leap_year(year);
  if is_leap {
    days += 1;
  } else if days == 365 {
    days = 0;
    year += 1;
  }

  let (month, day) = calculate_month_and_day(days, is_leap);
  (year, month, day, hours, minutes, seconds)
}

pub fn utc(sec: u64) -> String {
  let (year, month, day, hours, minutes, seconds) = ymdhms(sec);
  format!(
    "{}-{:02}-{:02}T{:02}:{:02}:{:02}",
    year, month, day, hours, minutes, seconds
  )
}

pub fn iso(sec: u64) -> String {
  let (year, month, day, hours, minutes, seconds) = ymdhms(sec);
  format!(
    "{}-{:02}-{:02} {:02}:{:02}:{:02}",
    year, month, day, hours, minutes, seconds
  )
}

pub fn ymd(sec: u64) -> String {
  let (year, month, day, ..) = ymdhms(sec);
  format!("{}-{:02}-{:02}", year, month, day)
}
