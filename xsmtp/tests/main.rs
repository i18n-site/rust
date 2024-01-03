#[tokio::test]
async fn test() -> Result<(), Box<dyn std::error::Error>> {
  async_send(
    "测试xsmtp",
    "i18n.site@foxmail.com",
    "测试邮件",
    Some("正文测试"),
    None,
  )
  .await?;
}
