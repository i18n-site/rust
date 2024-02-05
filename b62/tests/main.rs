#[test]
fn test() {
  let a = b"12345sdfghjuEFEFZ";
  let b = b62::e(a);
  let o = b62::d(&b).unwrap();
  assert_eq!(&a[..], &o[..]);
}
