use std::time::Duration;

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

genv::s!(WXPUSH_TOKEN, WXPUSH_ID);

#[allow(non_snake_case)]
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug)]
struct Message {
  pub appToken: String,
  pub summary: String,
  pub content: String,
  pub topicIds: Vec<String>,
  pub url: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseData {
  pub uid: Option<i64>,
  pub topicId: i64,
  pub messageId: i64,
  pub messageContentId: i64,
  pub sendRecordId: i64,
  pub code: i64,
  pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
  pub code: i64,
  pub msg: String,
  pub data: Vec<ResponseData>,
  pub success: bool,
}

#[derive(Error, Debug)]
pub enum WxPushError {
  #[error("wxpush response error: {0:?}")]
  Response(Response),
}

pub const SUCCESS: i64 = 1000;

fn cut<'a>(s: &'a str, max_length: usize) -> &'a str {
  if s.len() > max_length {
    let mut byte_count = 0;
    for c in s.chars() {
      let char_bytes = c.len_utf8();
      let t = byte_count + char_bytes;
      if t > max_length {
        break;
      }
      byte_count = t;
    }
    return &s[..byte_count];
  }
  s
}

pub async fn send(
  subject: impl AsRef<str>,
  content: impl AsRef<str>,
  url: impl AsRef<str>,
) -> Result<()> {
  let content = cut(content.as_ref(), 40000);
  let subject = cut(subject.as_ref(), 100);
  let url = cut(url.as_ref(), 400);

  let message = Message {
    appToken: WXPUSH_TOKEN.clone(),
    topicIds: vec![WXPUSH_ID.clone()],
    summary: format!("{subject}"),
    content: content.to_owned(),
    url: if url.is_empty() {
      None
    } else {
      Some(url.into())
    },
  };
  let client = Client::builder().timeout(Duration::from_secs(60)).build()?;

  let res = client
    .post("http://wxpusher.zjiecode.com/api/send/message")
    .header("content-type", "application/json")
    .json(&message)
    .send()
    .await?;

  let text = res.text().await?;
  dbg!(&text);
  let response: Response = sonic_rs::from_str(&text)?;
  if response.code != SUCCESS {
    return Err(WxPushError::Response(response))?;
  }

  Ok(())
}
