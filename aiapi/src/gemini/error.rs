use sonic_rs::Deserialize;

/*
{
  "error": {
    "code": 403,
    "message": "Permission denied: Consumer 'api_key:AIzaSyBkHAXfQaTjB3OG5r7tzKGS-SB7MDnYLqg' has been suspended.",
    "status": "PERMISSION_DENIED",
    "details": [
      {
        "@type": "type.googleapis.com/google.rpc.ErrorInfo",
        "reason": "CONSUMER_SUSPENDED",
        "domain": "googleapis.com",
        "metadata": {
          "containerInfo": "api_key:AIzaSyBkHAXfQaTjB3OG5r7tzKGS-SB7MDnYLqg",
          "consumer": "projects/278162933788",
          "service": "generativelanguage.googleapis.com"
        }
      },
      {
        "@type": "type.googleapis.com/google.rpc.LocalizedMessage",
     "locale": "en-US",
        "message": "Permission denied: Consumer 'api_key:AIzaSyBkHAXfQaTjB3OG5r7tzKGS-SB7MDnYLqg' has been suspended."
      }
    ]
  }
}
*/

#[derive(Deserialize, Debug)]
pub struct GeminiError {
  pub error: Error,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Error {
  pub code: u16,
  pub message: String,
  pub status: String,
  pub details: Vec<Detail>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Detail {
  #[serde(rename = "@type")]
  pub r#type: String,
  pub reason: Option<String>,
  pub domain: Option<String>,
  pub metadata: Option<Metadata>,
  pub locale: Option<String>,
  pub message: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
  #[serde(rename = "containerInfo")]
  pub container_info: String,
  pub consumer: String,
  pub service: String,
}
