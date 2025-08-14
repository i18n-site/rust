use reqwest::{Response, StatusCode};

use crate::Error;

pub async fn to_error(response: Response) -> Result<Error, reqwest::Error> {
  let status = response.status();
  let text = response.text().await?;
  Ok(match status {
    StatusCode::TOO_MANY_REQUESTS => Error::RateLimit { text },
    StatusCode::GATEWAY_TIMEOUT => Error::Timeout { text },
    StatusCode::BAD_REQUEST => Error::ApiKeyInvalid { text },
    _ => Error::Api { status, text },
  })
}
