use aok::{OK, Void};
use tracing::info;
use code_replace::code_replace;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  for i in [
    "x<code x=\"123\">abc</code>y<code x=\"123\">abc</code>",
    "a<code x=\"1\">bd</code>e",
    r#"<code v="0">快猫星云</code>告警平台"#,
  ] {
    let result = code_replace(i, |result, _org, id| result.push_str(id));
    info!("\n{}\n{}\n", i, result);
  }

  OK
}
