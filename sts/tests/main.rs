use sts::{day, month_day, now_month, now_month_day};

#[test]
fn main() {
  let m = now_month();
  dbg!(&m);
  let r = month_day(m);
  let day = day() as _;
  assert!(r.contains(&day));
  assert_eq!(r, now_month_day());
  dbg!(sts::n_ym(m));
}
