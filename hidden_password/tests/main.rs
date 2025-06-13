use aok::{OK, Void};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  let url = "postgresql://xxx:123456@us-east-2.aws.neon.tech/xx";
  let hidden = hidden_password::hidden_password(url);
  assert_eq!(hidden, "postgresql://xxx:***@us-east-2.aws.neon.tech/xx");
  info!("{}", hidden);
  OK
}
