use std::fmt;

use aok::Result;
use ireq::REQ;
use serde::Deserialize;
use sonic_rs::{JsonValueTrait, Value, from_slice, from_value};

#[derive(Debug)]
pub struct DohError {
  pub doh: String,
  pub msg: String,
}

impl fmt::Display for DohError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} : {}", self.doh, self.msg)
  }
}

impl std::error::Error for DohError {}

#[derive(Debug, Clone, Deserialize)]
pub struct Answer {
  pub name: String,
  pub r#type: u16,
  #[serde(alias = "TTL")]
  pub ttl: u64,
  pub data: String,
}

pub async fn post(doh: &str, query: &str) -> Result<Vec<Answer>> {
  let url = format!("https://{doh}{query}");

  let req = REQ.get(url).header("Accept", "application/dns-json");
  let res = ireq::req(req).await?;

  // tracing::info!(doh, "{}", String::from_utf8_lossy(&res));
  if let Ok::<Value, _>(json) = xerr::ok!(from_slice(&res))
    && let Some(status) = json.get("Status").as_u64()
    && status == 0
    && let Some(answer_li) = json.get("Answer")
    && let Ok(mut li) = xerr::ok!(from_value::<Vec<Answer>>(answer_li))
  {
    li.iter_mut().for_each(|i| {
      // TXT
      if i.r#type == 16 && i.data.starts_with('"') && i.data.ends_with('"') {
        i.data = i.data[1..i.data.len() - 1].into();
      }
    });
    return Ok(li);
  }

  Err(
    DohError {
      doh: doh.into(),
      msg: String::from_utf8_lossy(&res).into(),
    }
    .into(),
  )
}
