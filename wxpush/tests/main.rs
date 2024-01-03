#[tokio::test]
async fn test() -> anyhow::Result<()> {
  Ok(wxpush::send("https://atomgit.com/3ti", "测试推送", "正文\n测试").await?)
}
