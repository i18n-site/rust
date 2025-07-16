use aok::{OK, Void};
use tracing::info;
use txt_li::TxtLi;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test_restore() -> Void {
  let mut txt_li = TxtLi::new();
  txt_li.push_tran("1");
  txt_li.push_no_tran("2");
  txt_li.push_tran("3");
  txt_li.push_no_tran("4");
  info!("{}", txt_li.restore << txt_li.li);
  OK
}
