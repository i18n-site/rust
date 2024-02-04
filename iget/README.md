[‼️]: ✏️README.mdt

# iget

```rust
use aok::{Result, OK};

#[tokio::test]
async fn main() -> Result<()> {
  iget::Site::new(false, "https://www.win-rar.com/")
    .down(
      "fileadmin/winrar-versions/winrar/winrar-x64-624.exe",
      "down/test.zip",
    )
    .await?;
  OK
}
```
