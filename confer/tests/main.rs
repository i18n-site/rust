use aok::{OK, Void};
use confer::Confer;
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  let mut conf = Confer::new(
    r#"
a : 1


# xx
b : https://ab=c
  "#,
  );
  let a: u64 = conf.get("a", 2);
  let c: u64 = conf.get("c", 34);
  info!(a, c);
  info!("{conf}");
  conf.set("a", 5);
  conf.set("x", 9);
  info!("{conf}");
  dbg!(conf.str("a"));
  OK
}
