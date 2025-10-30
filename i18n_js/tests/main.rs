
use aok::{Result, OK};
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  // let home = home_dir().unwrap();
  // let root = home.join("i18n/md");
  // let build = Build::new(root.clone())?;
  // let vfs = build.build("dev").await?;
  // if vfs.save()? {
  //   dbg!("todo publish");
  // }
  OK
}
