genv::s!(TEST1);
genv::s!(TEST2: String);
genv::s!(TEST3: String, TEST4);
genv::def!(TEST5);
genv::def!(TEST6:u64 | 8080);

#[test]
fn test() {
  println!("{}", *TEST1);
  // let test1: u32 = TEST2();
  // println!("{}", TEST3());
}
