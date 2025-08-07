use aok::{OK, Void};

fn decode(a: u8) -> Void {
  let _ = String::from_utf8(vec![a])?;
  OK
}

#[test]
fn test() {
  assert!(decode(99).is_ok());
}
