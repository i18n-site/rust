use std::collections::HashSet;

use aiapi::Conf;
use aok::{OK, Void};
use dashmap::DashMap;
use futures::stream::StreamExt;
use pbar::pbar;

use crate::verify;

pub async fn verify_token_set(user_token_set_li: Vec<(String, HashSet<String>)>) -> Void {
  let conf = Conf {
    system: "".into(),
    temperature: 0.0,
  };

  let proxy_next = reqwest_client::proxy_iter();
  let err_li = &DashMap::<String, Vec<(String, String)>>::new();
  let token_sum = user_token_set_li.iter().map(|i| i.1.len() as u64).sum();
  let pbar = pbar(token_sum);
  futures::stream::iter(user_token_set_li.into_iter().flat_map(|(user, token_set)| {
    token_set
      .into_iter()
      .map(move |token| (user.clone(), token))
  }))
  .for_each_concurrent(64, |(user, token)| {
    let conf = &conf;
    let pbar = pbar.clone();
    let proxy = proxy_next();
    async move {
      if let Err(e) = verify(proxy, conf, &token).await {
        let e = e.to_string();
        tracing::error!(user = %user, token = %token, "{}", e);
        err_li.entry(user).or_default().push((token, e));
      }
      pbar.inc(1);
    }
  })
  .await;

  if err_li.is_empty() {
    println!("✅ {token_sum} 个令牌都有效");
  } else {
    for entry in err_li.iter() {
      let user = entry.key();
      let err_li = entry.value();
      println!("# {}", user);
      for (token, error) in err_li {
        println!("\ttoken: {} error: {}", token, error);
      }
    }
  }

  OK
}
