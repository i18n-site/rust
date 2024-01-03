#[tokio::test]
async fn test() -> anyhow::Result<()> {
  Ok(
    xsmtp::async_send(
      "测试xsmtp",
      "i18n.site@gmail.com",
      "测试邮件",
      "正文\n测试",
      "",
    )
    .await?,
  )
}
