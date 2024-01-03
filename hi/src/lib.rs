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

  tokio::join!(
    xsmtp::send(
      from_name,
      title,
      if url.is_empty() {
        url.to_owned() + "\n\n" + &txt;
      } else {
        txt
      },
      "",
    ),
    wxpush::send(title, txt, url)
  )
}
