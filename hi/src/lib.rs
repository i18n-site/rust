pub mod lark;

genv::s!(TO_MAIL, NAME);

// https://github.com/rust-lang/rust/issues/83527#issuecomment-1876263029
// macro_rules! join {
//     ($($task:expr),*) => {{
// let r = tokio::join!($($task,)*);
// xerr::log!($(r.${task.index()}),*);
//     }};
// }

pub async fn send(title: impl AsRef<str>, txt: impl AsRef<str>, url: impl AsRef<str>) {
  let name: &str = NAME.as_ref();
  let title = title.as_ref();
  let txt = txt.as_ref();
  let url = url.as_ref();

  let title_name = format!("{} · {}", title, name);

  let r = tokio::join!(
    xsmtp::send(
      name,
      TO_MAIL.as_ref(),
      title,
      if url.is_empty() {
        url.to_owned() + "\n\n" + txt
      } else {
        txt.to_owned()
      },
      "",
    ),
    wxpush::send(&title_name, txt, url),
    lark::send(&title_name, txt, url)
  );
  xerr::log!(r.0, r.1, r.2);
}
