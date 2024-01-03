use aok::Result;
pub async fn send(
  subject: impl AsRef<str>,
  content: impl AsRef<str>,
  url: impl AsRef<str>,
) -> Result<()> {
}
