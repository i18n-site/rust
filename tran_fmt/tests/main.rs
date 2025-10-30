use aok::{OK, Void};
use tracing::info;
use tran_fmt::{restore, tran_fmt};

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  let txt = r#"

测试

    文本


        正确


"#;
  let (line_pos, li) = tran_fmt(txt);

  let restored = restore(line_pos, li);
  info!("{}", restored);
  assert_eq!(txt.trim_end(), restored);
  OK
}
