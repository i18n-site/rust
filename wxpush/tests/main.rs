#[tokio::test]
async fn test() -> anyhow::Result<()> {
  let mut title = "测试推送";
  while title.len() < 4000 {
    title += title.as_str();
  }
  let mut body = "正文\n测试";
  while body.len() < 999999 {
    body += body.as_str();
  }

  let mut url = "https://atomgit.com/3ti";
  while url.len() < 4000 {
    url += url.as_str();
  }
  Ok(wxpush::send(url, title, body).await?)
}
