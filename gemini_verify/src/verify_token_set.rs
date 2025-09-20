use std::{collections::HashSet, sync::Arc};

use aiapi::Conf;
use aok::Result;
use dashmap::{DashMap, DashSet};
use futures::stream::StreamExt;
use pbar::pbar;
use reqwest::Client;

use crate::verify;

pub async fn verify_token_set(
  user_token_set_li: &[(String, HashSet<String>)],
) -> Result<DashSet<String>> {
  let conf = Conf {
    system: "".into(),
    temperature: 0.0,
  };

  let client = Arc::new(Client::builder().build()?);

  let proxy_next = reqwest_client::proxy_iter();
  let err_li = &DashMap::<String, Vec<(String, String)>>::new();
  let token_sum = user_token_set_li.iter().map(|i| i.1.len() as u64).sum();
  let pbar = pbar(token_sum);
  let ban = DashSet::new();
  let ban_ref = &ban;

  futures::stream::iter(
    user_token_set_li
      .iter()
      .flat_map(|(user, token_set)| token_set.iter().map(move |token| (user.clone(), token))),
  )
  .for_each_concurrent(4, |(user, token)| {
    let conf = &conf;
    let pbar = pbar.clone();
    let proxy = proxy_next();
    let client = client.clone();
    async move {
      if verify(proxy, conf, &token).await.is_err()
        && let Err(e) = verify(&client, conf, &token).await
      {
        if let Some(e) = e.downcast_ref::<aiapi::Error>()
          && let aiapi::Error::Gemini(err) = e
        {
          let err = &err.error;
          if err.code == 403 && err.message.contains("suspended") {
            ban_ref.insert(token.into());
            return;
          }
        }
        let e = e.to_string();
        log::error!("{user} {token} {e}");
        err_li.entry(user).or_default().push((token.into(), e));
      }
      pbar.inc(1);
    }
  })
  .await;

  for entry in err_li.iter() {
    let user = entry.key();
    let err_li = entry.value();
    let mut fmtted = format!("# {user}\n");
    for (token, error) in err_li {
      fmtted.push_str(&format!("\ttoken: {token} error: {error}\n"));
    }
    println!("{}", fmtted);
  }

  Ok(ban)
}
