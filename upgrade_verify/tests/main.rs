use aok::{OK, Void};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  upgrade_verify::check(
    vb::e([0, 2, 1]),
    "/var/folders/4x/n5x1dwrj39v_hl53vyprqcn80000gn/T/i18/0.2.1/node_modules.tar",
    std::fs::read("/Users/z/host/conf/env/upgrade/pk")?
      .try_into()
      .unwrap(),
  )?;

  OK
}
