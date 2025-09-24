use std::{future::Future, pin::Pin};

use aok::Void;
use futures::{FutureExt, StreamExt, stream::FuturesUnordered};
pub mod lark;

genv::s!(HI_MAILTO, HI_NAME);

#[static_init::dynamic]
static WX_TOKEN_TOPIC_ID: (String, u64) =
  sonic_rs::from_slice(std::env::var("WxPush").expect("miss env WxPush").as_bytes())
    .expect("env WxPush invalid");

pub async fn send(title: impl AsRef<str>, txt: impl AsRef<str>, url: impl AsRef<str>) {
  let name: &str = HI_NAME.as_ref();
  let title = title.as_ref();
  let txt = txt.as_ref();
  let url = url.as_ref();

  let mut futures: FuturesUnordered<Pin<Box<dyn Future<Output = Void> + Send + 'static>>> =
    FuturesUnordered::new();

  futures.push(Box::pin(xsmtp::send(
    name,
    HI_MAILTO.as_ref(),
    title.to_owned(),
    if url.is_empty() {
      txt.to_owned()
    } else {
      url.to_owned() + "\n\n" + txt
    },
    "",
  )));

  futures.push(
    wxpush::send(
      &WX_TOKEN_TOPIC_ID.0,
      [WX_TOKEN_TOPIC_ID.1],
      title.to_owned(),
      txt.to_owned(),
      url.to_owned(),
    )
    .boxed(),
  );

  futures.push(lark::send(title.to_owned(), txt.to_owned(), url.to_owned()).boxed());

  while let Some(result) = futures.next().await {
    match result {
      Ok(_) => {}
      Err(err) => tracing::error!("{err}"),
    }
  }
}
