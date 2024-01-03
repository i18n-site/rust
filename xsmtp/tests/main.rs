#[tokio::test]
async fn test() -> anyhow::Result<()> {
  Ok(
    xsmtp::async_send(
      "测试xsmtp",
      "i18n.site@foxmail.com",
      "测试邮件",
      "正文\n测试",
      "",
    )
    .await?,
  )
}
