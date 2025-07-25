use aok::{OK, Result};

#[tokio::test]
async fn main() -> Result<()> {
  // let github = iget::Site::new(true, "https://github.com/i18n-site/bin/releases/download/");

  // let txt = github.txt("_/v").await?;

  let bar = iget::Site::new("https://www.win-rar.com/")
    .down(
      "fileadmin/winrar-versions/winrar/winrar-x64-624.exe",
      "down/test.zip",
    )
    .await?;
  bar.show().await?;
  OK
}
