use aok::{OK, Void};
use tracing::info;
use ver_from_txt::ver_from_txt;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  let txt = "AAEp;Gup51/v;up[0,2~3].u-01.eu.org;yutk.eu.org";

  let r = ver_from_txt("i18", &[0, 0, 1], txt)?;
  info!("{:?}", r);
  OK
}
