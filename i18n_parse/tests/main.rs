use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let s = "xxxx${ I18N.doc }xx${I18N.xxx}x";
  let r = i18n_parse::extract(s);
  let to = ["Document", "Example"];

  let r = i18n_parse::replace(s, &r.range[..], &to);
  dbg!(r);
  OK
}
