use aok::{OK, Result};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  // let s = "12345${ I18N.doc } ${12345678}12${I18N.xxx}12";
  // let r = rvar::extract(s);

  // let r = rvar::replace(s, &r.range[..], |key| {
  //   dbg!(key);
  //   key[5..].into()
  // });
  // dbg!(r);
  OK
}
