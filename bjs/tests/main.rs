use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  // let root = std::env!("CARGO_MANIFEST_DIR");
  //
  // let root = format!("{root}/tests");
  // let ctx = &mut bjs::ctx(&root, &root);
  // let mut map1 = HashMap::new();
  // map1.insert("key1".to_string(), "value1".to_string());
  // map1.insert("key2".to_string(), "value2".to_string());
  //
  // let mut map2 = HashMap::new();
  // map2.insert("key3".to_string(), "value3".to_string());
  // map2.insert("key4".to_string(), "value4".to_string());
  //
  // let arg = [
  //   bjs::li_hashmap_to_jsvalue(ctx, &[map1, map2]),
  //   bjs::li_str_to_jsvalue(ctx, &["x", "b"]),
  // ];
  //
  // match bjs::default(ctx, format!("{root}/test.js"), &arg) {
  //   Ok(r) => {
  //     // dbg!(bjs::VecOrStr::parse(r, ctx));
  //     let r = bjs::obj2map(r).unwrap();
  //     dbg!(r);
  //     // dbg!(&r);
  //     // if let Some(file) = r.remove("file") {
  //     //   dbg!(bjs::li_str(ctx, file));
  //     // }
  //   }
  //   Err(err) => {
  //     info!("{}", err);
  //   }
  // }
  OK
}
