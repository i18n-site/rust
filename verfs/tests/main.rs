use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

// #[tokio::test]
// async fn test() -> Result<()> {
//   info!("{}", 123456);
//   OK
// }

#[test]
fn test() -> Result<()> {
  // let dir: PathBuf = env!("CARGO_MANIFEST_DIR").into();
  // let yml = dir.join("tests/test.yml");
  // let mut hash = HashMap::new();
  // let r = latest_ver(&yml, &mut hash)?;
  // dbg!(r);
  // let tests = dir.join("tests");
  //
  // let mut verfs = VerFs::load(&tests, tests.join("out"), tests.join("log"))?;
  //
  // verfs.cp("main.rs")?;
  // verfs.wstr("a/b/c.txt", "123\n456")?;
  // verfs.wstr("ab/b/c.txt", "123\n456")?;
  // verfs.wstr("b/1/2.txt", "123\n")?;
  // verfs.wstr("b/2/2.txt", "123\n456")?;
  // verfs.save()?;
  //
  // let mut prefix_li = PrefixLi::new(vec!["a".into(), "b".into()]);
  //
  // for (k, v) in verfs.sorted_rel_ver() {
  //   info!("{k} {v}");
  //   prefix_li.add(k, v);
  // }
  // for (prefix, i) in prefix_li.0 {
  //   println!("{prefix} {}", to_string(&i)?);
  // }

  OK
}
