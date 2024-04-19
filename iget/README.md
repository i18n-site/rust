[‼️]: ✏️README.mdt

# iget

```rust
use aok::{Result, OK};

#[tokio::test]
async fn main() -> Result<()> {
  // let github = iget::Site::new(true, "https://github.com/i18n-site/bin/releases/download/");

  // let txt = github.txt("_/v").await?;
  // dbg!(txt);

  let bar = iget::Site::new(false, "https://www.win-rar.com/")
    .down(
      "fileadmin/winrar-versions/winrar/winrar-x64-624.exe",
      "down/test.zip",
    )
    .await?;
  bar.show().await?;
  OK
}
```
