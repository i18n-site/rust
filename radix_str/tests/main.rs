use radix_str::radix_str;

static TEST0: &str = radix_str!(1234567890, 36);
static TEST1: &str = radix_str!(-1234567890, 36);

#[test]
fn test() {
  assert_eq!(TEST0, "kf12oi");
  assert_eq!(TEST1, "-kf12oi");
  println!("TEST0 = {}", TEST0);
  println!("TEST1 = {}", TEST1);
}
