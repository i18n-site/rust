#[tokio::test]
async fn test() -> anyhow::Result<()> {
  let mut title = "测试推送".to_owned();
  while title.len() < 4000 {
    title.push(title);
  }
  let mut body = "正文\n测试\n".to_owned();
  while body.len() < 999999 {
    body.push(body);
  }

  let mut url = "https://atomgit.com/3ti".to_owned();
  while url.len() < 4000 {
    url.push("123456790");
  }
  Ok(wxpush::send(&url, &title, &body).await?)
}
