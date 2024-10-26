// use sts::{day, month_day, now_month, now_month_day};

#[test]
fn main() {
  // let mut now = sts::sec();
  // // let m = now_month();
  // // let r = month_day(m);
  // // let day = day() as _;
  // // assert!(r.contains(&day));
  // // assert_eq!(r, now_month_day());
  //
  // let mut n = 0;
  // while n < 86400 * 373 * 100 {
  //   n += 3600 * 24;
  //   let s = sts::utc(now + n - 86400 * 365);
  //   let n: u8 = s
  //     .split("T")
  //     .next()
  //     .unwrap()
  //     .split("-")
  //     .last()
  //     .unwrap()
  //     .parse()
  //     .unwrap();
  //   assert!(n <= 31);
  // }
  // println!("\n{}\n", sts::utc(now));
}
