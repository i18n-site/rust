use std::time::Duration;

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use xstr::cut;

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

pub async fn send(
  token: impl AsRef<str>,
  topic_ids: impl AsRef<[u64]>,
  subject: impl AsRef<str>,
  content: impl AsRef<str>,
  url: impl AsRef<str>,
) -> Result<()> {
  let subject = cut(subject.as_ref(), 100);
  let content = content.as_ref();
  let content = if !subject.is_empty() {
    subject.to_owned() + "\n" + content
  } else {
    content.to_owned()
  };
  let content = cut(content.as_ref(), 40000);
  let url = cut(url.as_ref(), 400);

  let message = Message {
    appToken: token.as_ref().into(),
    topicIds: topic_ids.as_ref().iter().map(|id| id.to_string()).collect(),
    summary: subject.to_string(),
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
  let response: Response = sonic_rs::from_str(&text)?;
  if response.code != SUCCESS {
    return Err(WxPushError::Response(response))?;
  }

  Ok(())
}
