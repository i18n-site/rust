use aok::{OK, Result};

#[test]
fn test() -> Result<()> {
  let li = [("D", 16), ("-G", 151), ("-!$x", 9553416)];
  for i in li {
    let n = intbin::bin_u64(burl::d(i.0)?);
    println!("{} = {}", i.0, n);
    assert_eq!(i.1, n);
  }

  OK
}
