use aok::{OK, Void, err, throw};

fn decode(a: u8) -> Void {
  let _ = String::from_utf8(vec![a])?;
  OK
}

fn test_err() -> Void {
  err!("This is an error")
}

fn test_throw() -> Void {
  throw!("This is a thrown error");
}

fn test_throw_with_arg() -> Void {
  throw!("Error with arg: {}", 42);
}

#[test]
fn test() {
  assert!(decode(99).is_ok());
  assert!(decode(128).is_err());
  assert!(test_err().is_err());
  assert!(test_throw().is_err());
  assert!(test_throw_with_arg().is_err());
}
