use std::{
  borrow::Borrow,
  sync::atomic::{AtomicUsize, Ordering},
};

use rand::Rng;
use tokio::sync::Semaphore;

use crate::{AiApi, ChatResult, ConfTrait, Error, Result};
pub const RESET: usize = usize::MAX / 2;

#[derive(Debug)]
pub struct TokenLi<AiApi> {
  pub token_li: Vec<String>,
  pub token_pos: AtomicUsize,
  pub aiapi: AiApi,
  pub semaphore: Semaphore,
}

impl<T: AiApi> TokenLi<T> {
  pub fn new<S: Into<String>>(
    token_li: impl IntoIterator<Item = S>,
    concurrent: usize,
    aiapi: T,
  ) -> Self {
    let token_li = token_li.into_iter().map(|s| s.into()).collect::<Vec<_>>();
    let token_li_len = token_li.len();
    Self {
      semaphore: Semaphore::new(concurrent * token_li_len),
      token_pos: AtomicUsize::new(rand::rng().random_range(0..token_li_len)),
      token_li,
      aiapi,
    }
  }

  pub async fn chat(
    &self,
    conf: &impl ConfTrait,
    model: &str,
    prompt: impl Into<String>,
  ) -> Result<ChatResult> {
    // Acquire a permit. It will be released when `_permit` goes out of scope (RAII).
    let _permit = self.semaphore.acquire().await;
    let mut pos = self.token_pos.fetch_add(1, Ordering::Relaxed);
    // tracing::info!("{pos}");
    if pos > RESET {
      self.token_pos.store(0, Ordering::Relaxed);
    }
    let token_li_len = self.token_li.len();
    let req = self.aiapi.req(conf.borrow(), model, prompt)?;
    let aiapi = &self.aiapi;

    let mut retry = 3;
    loop {
      let token = &self.token_li[pos % token_li_len];
      match aiapi.chat(token, &req).await {
        Ok(mut r) => {
          r.content = conf.fmt(r.content);
          return Ok(r);
        }
        Err(e) => {
          if retry > 0 {
            pos = pos.overflowing_add(1).0;
            retry -= 1;
            if matches!(
              e,
              Error::Timeout { .. }
                | Error::RateLimit { .. }
                | Error::ApiKeyInvalid { .. }
                | Error::EmptyResponse { .. }
            ) {
              tracing::warn!("{} {token}\n{e}", aiapi.url());
              continue;
            }
          }
          tracing::error!("{} {token}\n{e}", aiapi.url());
          return Err(e);
        }
      }
    }
  }
}
