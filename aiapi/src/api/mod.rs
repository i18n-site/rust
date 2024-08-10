include!(concat!(env!("OUT_DIR"), "/api.rs"));
use aok::Result;
use reqwest::{RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use tracing::warn;
mod baidu;
pub use baidu::Baidu;

use crate::{
  msg::MsgLi,
  response::{ApiError, OpenAiResponse, Response},
};
mod token;
pub use token::Token;
pub mod token_model;
pub use token_model::TokenModel;

pub fn url_chat_completions(api: impl AsRef<str>) -> String {
  format!("{}/chat/completions", api.as_ref())
}

pub fn temperature(s: impl AsRef<str>) -> String {
  format!(r#""temperature":{}"#, s.as_ref())
}

pub fn system_msg(system: impl AsRef<str>, mut msg: String) -> String {
  let system = system.as_ref();
  if !system.is_empty() {
    let system = sonic_rs::to_string(&system).unwrap();
    msg = format!(r#"{{"role":"system","content":{system}}},"#) + &msg;
  }
  msg
}

pub async fn post<R: DeserializeOwned>(
  url: impl AsRef<str>,
  body: impl Into<String>,
  headers: impl Fn(RequestBuilder) -> RequestBuilder,
) -> Result<R> {
  let url = url.as_ref();
  let body = body.into();
  let req = ireq::REQ.post(url).body(body.clone());
  let response = headers(req).send().await?;

  let status = response.status();
  let msg = response.text().await?;

  if status == StatusCode::OK {
    match sonic_rs::from_str(&msg) {
      Ok(r) => Ok(r),
      Err(err) => Err(ApiError::DecodeError { msg, err }.into()),
    }
  } else {
    Err(
      ApiError::RequestError {
        status,
        url: url.into(),
        msg,
      }
      .into(),
    )
  }
}

async fn openai<T: OpenAi + ?Sized>(
  api: &T,
  system: impl AsRef<str>,
  msg: String,
) -> Result<OpenAiResponse> {
  let max_tokens = api.max_tokens();
  let msg = system_msg(system, msg);
  let arg = api.arg();

  let mut retry_count = 0;

  loop {
    retry_count += 1;
    let (token, model) = api.token_model();
    let body =
      format!(r#"{{{arg},"max_tokens":{max_tokens},"model":"{model}","messages":[{msg}]}}"#);
    let url = api.url_chat_completions();
    let r = post(url, body, |req| {
      req
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
    })
    .await;
    match r {
      Ok(r) => return Ok(r),
      Err(err) => {
        if retry_count < 9
          && let Some(err) = err.downcast_ref::<ApiError>()
          && let ApiError::RequestError { status, msg, .. } = &err
          && status == &StatusCode::TOO_MANY_REQUESTS
        {
          warn!("{retry_count} {status} {url}\n{msg}");
          tokio::time::sleep(std::time::Duration::from_secs(20)).await;
          continue;
        }
        if retry_count > 3 {
          return Err(err);
        }
      }
    }
  }
}

pub trait OpenAi {
  fn max_tokens(&self) -> usize;
  fn 上下文(&self) -> usize;
  fn token_model(&self) -> (&str, &str);
  fn arg(&self) -> &str;
  fn url_chat_completions(&self) -> &str;
}

impl<T: OpenAi + Sync> Api for T {
  async fn send(
    &self,
    system: impl Into<String> + Send,
    msg_li: impl Into<MsgLi> + Send,
  ) -> Result<Response> {
    let msg: String = msg_li.into().into();
    let system = system.into();
    let r: OpenAiResponse = openai(self, system, msg).await?;
    Ok(r.into())
  }
  fn max_tokens(&self) -> usize {
    self.max_tokens()
  }
  fn 上下文(&self) -> usize {
    self.上下文()
  }
}

pub trait Api {
  fn send(
    &self,
    system: impl Into<String> + Send,
    msg_li: impl Into<MsgLi> + Send,
  ) -> impl std::future::Future<Output = Result<Response>> + Send;
  fn 上下文(&self) -> usize;
  fn max_tokens(&self) -> usize;
}

#[derive(Debug)]
pub enum EnumApi {
  Token(Token),
  TokenModel(TokenModel),
  Baidu(Baidu),
}

impl Api for EnumApi {
  fn 上下文(&self) -> usize {
    match self {
      EnumApi::Token(t) => Api::上下文(t),
      EnumApi::TokenModel(t) => Api::上下文(t),
      EnumApi::Baidu(t) => Api::上下文(t),
    }
  }
  fn max_tokens(&self) -> usize {
    match self {
      EnumApi::Token(t) => Api::max_tokens(t),
      EnumApi::TokenModel(t) => Api::max_tokens(t),
      EnumApi::Baidu(t) => Api::max_tokens(t),
    }
  }
  async fn send(
    &self,
    system: impl Into<String> + Send,
    msg_li: impl Into<MsgLi> + Send,
  ) -> Result<Response> {
    Ok(match self {
      EnumApi::Token(t) => Api::send(t, system, msg_li).await?,
      EnumApi::TokenModel(t) => Api::send(t, system, msg_li).await?,
      EnumApi::Baidu(t) => Api::send(t, system, msg_li).await?,
    })
  }
}
