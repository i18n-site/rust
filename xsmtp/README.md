[‼️]: ✏️README.mdt

# xsmtp

```rust
#[tokio::test]
async fn test() -> anyhow::Result<()> {
  Ok(
    xsmtp::send(
      "测试xsmtp",
      "i18n.site@gmail.com",
      "测试邮件",
      "正文\n测试",
      "",
    )
    .await?,
  )
}
```
