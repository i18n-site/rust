use aok::OK;

#[tokio::test]
async fn test() -> aok::Result<()> {
  hi::send("测试标题", "测试内容", "https://baidu.com").await;
  OK
}
