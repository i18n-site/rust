#[test]
fn test() {
  for num in [u64::MAX, 1234567890, 0, 1] {
    #[cfg(feature = "url")]
    {
      use rany::URL;
      let encode = URL.estr(num);
      let decode = URL.dstr(&encode);
      assert_eq!(num, decode);
    }
    #[cfg(feature = "b255")]
    {
      use rany::{Rany, B255};
      let encode = B255.e(num);
      assert_eq!(num, B255.d(&encode));
    }
  }
}
