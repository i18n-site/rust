use only_url::only_url;
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

// #[tokio::test]
// async fn test() -> Result<()> {
//   info!("{}", 123456);
//   OK
// }

#[test]
fn main() {
  fn test(s: &str) {
    info!("{s} {}", only_url(s));
  }
  test(" ![ ](content) !   ");
  test(" x ![ ](content) !   ");
  test(" ![x](content)");
  test(" [](content)");
  test(" abc");
  test("abc");
  test("[x](xxx)");
  test("https://www.seafile.com/download/")
}
