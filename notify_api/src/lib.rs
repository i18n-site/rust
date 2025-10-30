use std::collections::HashMap;

use aok::{OK, Void};

#[static_init::dynamic]
pub static NOTIFY_API: Vec<String> = {
  let li = std::env::var("NotifyApi").unwrap_or_else(|_| {
    panic!("NO ENV NotifyApi");
  });
  li.split_whitespace().map(String::from).collect()
};

pub async fn send(title: impl AsRef<str>, txt: impl AsRef<str>, url: impl AsRef<str>) -> Void {
  let mut body = HashMap::new();

  macro_rules! insert {
    ($($name:ident),+) => {{
      $(
        let $name = $name.as_ref();
        if !$name.is_empty() {
          body.insert(stringify!($name), $name);
        }
      )+
    }};
  }
  insert!(title, txt, url);

  let mut n = NOTIFY_API.len();
  if let Ok(body) = xerr::ok!(sonic_rs::to_string(&body)) {
    for api in NOTIFY_API.iter() {
      if let Err(err) = ireq::post(api, body.clone()).await {
        tracing::error!("{api} {err}");
        n -= 1;
        if n == 0 {
          return Err(err);
        }
      }
    }
  }
  OK
}
