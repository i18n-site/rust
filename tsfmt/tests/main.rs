use chrono::NaiveDate;
use tsfmt::tsfmt;

fn is_valid_date(date_str: &str, ts: u64) -> bool {
  match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
    Ok(_) => true,
    Err(_) => {
      eprintln!("ts {ts} INVALID DATE {}", date_str);
      false
    }
  }
}

#[test]
fn main() {
  let now = sts::sec();

  let mut n = 0;
  while n < 86400 * 373 * 100 {
    n += 3600;
    let ts = now + n;
    let s = tsfmt(ts);
    let date = s.split("T").next().unwrap();
    assert!(is_valid_date(date, ts));
  }
  println!("\n{}\n", tsfmt(now));
}
