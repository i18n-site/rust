pub mod lark;

use aok::{Result, OK};

genv::s!(TO_MAIL);

pub async fn send(
  from_name: impl AsRef<str>,
  title: impl AsRef<str>,
  txt: impl AsRef<str>,
  url: impl AsRef<str>,
) -> Result<()> {
  let from_name = from_name.as_ref();
  let title = title.as_ref();
  let txt = txt.as_ref();
  let url = url.as_ref();

  let result_li = tokio::join!(
    xsmtp::send(
      from_name,
      TO_MAIL.as_ref(),
      title,
      if url.is_empty() {
        url.to_owned() + "\n\n" + txt
      } else {
        txt.to_owned()
      },
      "",
    ),
    wxpush::send(
      if from_name.is_empty() {
        title.to_owned()
      } else {
        from_name.to_owned() + " " + title
      },
      txt,
      url
    )
  );
  xerr::log!(result_li.0, result_li.1);
  OK
}
