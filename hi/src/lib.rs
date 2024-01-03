use aok::Result;

pub async fn send(
  from_name: impl AsRef<str>,
  title: impl AsRef<str>,
  txt: impl AsRef<str>,
  url: impl AsRef<str>,
) -> Result<()> {
  let title = title.as_ref();
  let txt = txt.as_ref().to_owned();
  let url = url.as_ref();

  let mut mail_txt = txt;
  if !url.is_empty() {
    mail_txt = url + "\n" + txt;
  }

  xsmtp::async_send(from_name, title, mail_txt, "").await;
  wxpush::send(title, txt, url)
}
