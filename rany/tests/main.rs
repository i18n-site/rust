#[test]
fn test() {
  use rany::RANY_URL_ID as rany;
  for num in [1234567890, 0, 1] {
    let encode = rany.e(num);
    let decode = rany.d(&encode);
    dbg!((&encode, &decode));
    assert_eq!(num, decode);
  }
}
