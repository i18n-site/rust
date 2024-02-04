use xhash::hash128;

#[test]
fn test() {
  let mut bin = 0u128.to_le_bytes();
  for i in 1..10000 {
    let r = hash128(&bin);
    dbg!((i, r));
    bin = r.to_le_bytes();
  }
}

/*
#[cfg(feature = "macro")]
mod test_macro {
}
*/
