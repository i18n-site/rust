[‼️]: ✏️README.mdt

# xsmtp

```rust
#[tokio::test]
async fn test() -> anyhow::Result<()> {
  xsmtp::send(
    "测试xsmtp 1",
    "i18n.site@gmail.com",
    "测试邮件 1",
    "正文\n测试",
    "",
  )
  .await?;
  xsmtp::send(
    "测试xsmtp 2",
    "i18n.site@gmail.com",
    "测试邮件 2",
    "正文\n测试",
    "",
  )
  .await
}
```
