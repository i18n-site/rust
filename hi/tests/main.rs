use aok::OK;

#[tokio::test]
async fn test() -> aok::Result<()> {
  hi::send("i18n.site", "测试标题", "测试内容", "https://baidu.com").await?;
  OK
}
