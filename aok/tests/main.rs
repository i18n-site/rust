use aok::{Null, OK};

fn decode(a: u8) -> Null {
  let _ = String::from_utf8(vec![a])?;
  OK
}

#[test]
fn test() {
  assert!(decode(99).is_ok());
}
