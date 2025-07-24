use aok::{OK, Void};
use find_close::find_close;
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  let htm = "abc<code>代码</code>123</code>测试";
  let pos = find_close(htm, "code");
  info!("{}", &htm[..pos]);
  OK
}
