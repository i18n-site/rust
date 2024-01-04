use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
  #[serde(rename = "Subject")]
  pub subject: String,
  #[serde(rename = "body-plain")]
  pub txt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
  pub payload: Payload,
}
