use aok::{OK, Void};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test() -> Void {
  let replacer = tag_replace::TagReplace::new("code", "v");
  for i in [
    "x<code x=\"123\">abc</code>y<code x=\"123\">abc</code>",
    "a<code x=\"1\">bd</code>e",
    r#"<code v="0">快猫星云</code>告警平台"#,
  ] {
    let result = replacer.replace(i, |result, _org, id| result.push_str(id));
    info!("\n{}\n{}\n", i, result);
  }

  OK
}
