use aok::{OK, Void};
use find_close::FindClose;
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  // let mut f = FindClose::new("code");
  // let htm = "abc<code>代码</code>123</code>测试";
  let mut f = FindClose::new("br");
  let htm = "<br />";
  let pos = f.find(htm).unwrap();
  info!("{}", &htm[..pos]);
  OK
}
